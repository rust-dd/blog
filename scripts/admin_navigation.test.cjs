const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');
const source = readFileSync(`${__dirname}/../assets/admin-navigation.js`, 'utf8');

function browser() {
  const listeners = {};
  const location = { href: '/admin' };
  const entries = [{ state: [12, 34], url: '/admin' }];
  let cursor = 0;
  let updates = 0;
  let confirmations = 0;
  let pendingTraversals = 0;
  const window = {};
  function element() {
    const handlers = {};
    return {
      addEventListener(name, callback) { (handlers[name] ??= new Set()).add(callback); },
      removeEventListener(name, callback) { handlers[name]?.delete(callback); },
      fire(name) { [...(handlers[name] ?? [])].forEach(callback => callback()); },
      showModal() { this.open = true; },
      close() { this.open = false; this.fire('close'); },
    };
  }
  const dialog = element(), stay = element(), leave = element();
  const elements = { 'admin-unsaved-dialog': dialog, 'admin-unsaved-stay': stay, 'admin-unsaved-leave': leave };
  function dispatch(name, state) {
    const event = { state, stopImmediatePropagation() { this.stopped = true; } };
    for (const callback of listeners[name] ?? []) {
      callback(event);
      if (event.stopped) break;
    }
    return event;
  }
  const history = {
    get state() { return entries[cursor].state; },
    pushState(state, _, url) {
      entries.splice(cursor + 1);
      entries.push({ state: structuredClone(state), url });
      cursor++;
      location.href = url;
    },
    replaceState(state, _, url) {
      entries[cursor] = { state: structuredClone(state), url: url ?? location.href };
      location.href = entries[cursor].url;
    },
    go(delta) {
      pendingTraversals++;
      setImmediate(() => {
        pendingTraversals--;
        cursor += delta;
        location.href = entries[cursor].url;
        dispatch('popstate', history.state);
      });
    },
  };
  window.addEventListener = (name, callback, capture) => {
    listeners[name] ??= [];
    if (capture) listeners[name].unshift(callback);
    else listeners[name].push(callback);
  };
  // The guard must precede the router even when the router registers first.
  window.addEventListener('popstate', () => updates++);
  vm.runInNewContext(source, { window, document: { addEventListener() {}, getElementById: id => elements[id] }, history, location });
  const confirm = window.rdAdminConfirm;
  window.rdAdminConfirm = async () => { confirmations++; return window.accept; };
  return { window, history, location, entries, dialog, stay, leave, confirm,
    settled: async () => {
      for (let turn = 0; turn < 20; turn++) {
        await new Promise(resolve => setImmediate(resolve));
        if (pendingTraversals === 0) return;
      }
      throw new Error('History restoration did not settle');
    },
    updates: () => updates, confirmations: () => confirmations };
}

test('canceling Back preserves the editor and history without notifying the router', async () => {
  const page = browser();
  page.history.pushState([0, 0], '', '/admin/new');
  page.history.replaceState([50, 60], '', undefined);
  page.window.rdAdminDirty = true;
  page.history.go(-1);
  await page.settled();
  assert.equal(page.location.href, '/admin/new');
  assert.equal(page.updates(), 0);
  assert.equal(page.confirmations(), 1);
  assert.equal(page.entries.length, 2);
  assert.equal(page.history.state[0], 50);
  assert.equal(page.history.state[1], 60);
});

test('accepted Back and canceled Forward preserve history and warn once', async () => {
  const page = browser();
  page.history.pushState([0, 0], '', '/admin/edit/first');
  page.history.pushState([0, 0], '', '/admin/edit/second');
  page.window.rdAdminDirty = true;
  page.window.accept = true;
  page.history.go(-1);
  await page.settled();
  assert.equal(page.location.href, '/admin/edit/first');
  assert.equal(page.updates(), 1);
  assert.equal(page.window.rdAdminDirty, false);
  page.window.rdAdminDirty = true;
  page.window.accept = false;
  page.history.go(1);
  await page.settled();
  assert.equal(page.location.href, '/admin/edit/first');
  assert.equal(page.updates(), 1);
  assert.equal(page.confirmations(), 2);
});

test('canceling a multi-entry traversal returns to the same draft', async () => {
  const page = browser();
  page.history.pushState([0, 0], '', '/admin/edit/first');
  page.history.pushState([0, 0], '', '/admin/edit/second');
  page.window.rdAdminDirty = true;
  page.history.go(-2);
  await page.settled();
  assert.equal(page.location.href, '/admin/edit/second');
  assert.equal(page.updates(), 0);
  assert.equal(page.entries.length, 3);
});

test('the custom dialog preserves the draft until an explicit leave decision', async () => {
  const page = browser();
  page.window.rdAdminConfirm = page.confirm;
  page.history.pushState([0, 0], '', '/admin/new');
  page.window.rdAdminDirty = true;
  page.history.go(-1);
  await page.settled();
  assert.equal(page.dialog.open, true);
  assert.equal(page.location.href, '/admin/new');
  page.stay.fire('click');
  await page.settled();
  assert.equal(page.dialog.open, false);
  assert.equal(page.window.rdAdminDirty, true);
  assert.equal(page.updates(), 0);
  page.history.go(-1);
  await page.settled();
  page.leave.fire('click');
  await page.settled();
  assert.equal(page.location.href, '/admin');
  assert.equal(page.window.rdAdminDirty, false);
  assert.equal(page.updates(), 1);
});

test('Escape cancels the sign-out dialog and a later confirmation resolves separately', async () => {
  const page = browser();
  const first = page.confirm('signout');
  assert.equal(page.leave.textContent, 'Sign out without saving');
  page.dialog.fire('cancel');
  assert.equal(await first, false);
  const second = page.confirm('leave');
  assert.equal(page.leave.textContent, 'Leave without saving');
  page.leave.fire('click');
  assert.equal(await second, true);
  assert.equal(page.dialog.open, false);
});
