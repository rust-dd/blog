(() => {
  if (window.rdAdminNavigation) return;
  window.rdAdminNavigation = true;
  const key = '__rdAdminIndex';
  const push = history.pushState.bind(history);
  const replace = history.replaceState.bind(history);
  let index = history.state?.[key] ?? 0;
  let restoring = null;
  let navigationPending = false;
  let restoreWaiters = [];
  let confirmation = null;

  window.rdAdminConfirm = kind => {
    if (confirmation) return confirmation;
    const dialog = document.getElementById('admin-unsaved-dialog');
    if (!dialog) return Promise.resolve(false);
    const stay = document.getElementById('admin-unsaved-stay');
    const leave = document.getElementById('admin-unsaved-leave');
    leave.textContent = kind === 'signout' ? 'Sign out without saving' : 'Leave without saving';
    confirmation = new Promise(resolve => {
      const finish = accepted => {
        stay.removeEventListener('click', cancel);
        leave.removeEventListener('click', accept);
        dialog.removeEventListener('cancel', cancel);
        dialog.removeEventListener('close', cancel);
        dialog.close();
        confirmation = null;
        resolve(accepted);
      };
      const cancel = () => finish(false);
      const accept = () => finish(true);
      stay.addEventListener('click', cancel);
      leave.addEventListener('click', accept);
      dialog.addEventListener('cancel', cancel);
      dialog.addEventListener('close', cancel);
      dialog.showModal();
    });
    return confirmation;
  };

  function tagged(state, position) {
    const copy = Array.isArray(state) ? Object.assign(state.slice(), state) : { ...state };
    copy[key] = position;
    return copy;
  }

  replace(tagged(history.state, index), '', location.href);
  history.pushState = (state, title, url) => {
    const next = index + 1;
    push(tagged(state, next), title, url);
    index = next;
  };
  history.replaceState = (state, title, url) => replace(tagged(state, index), title, url);

  window.addEventListener('popstate', event => {
    const next = event.state?.[key];
    if (!Number.isInteger(next)) return;
    if (restoring !== null) {
      event.stopImmediatePropagation();
      if (next === restoring) {
        restoring = null;
        const waiting = restoreWaiters;
        restoreWaiters = [];
        waiting.forEach(resolve => resolve());
      }
      else history.go(restoring - next);
      return;
    }
    if (window.rdAdminDirty && next !== index) {
      event.stopImmediatePropagation();
      restoring = index;
      history.go(index - next);
      if (navigationPending || confirmation) return;
      navigationPending = true;
      window.rdAdminConfirm('leave').then(async accepted => {
        if (restoring !== null) await new Promise(resolve => restoreWaiters.push(resolve));
        navigationPending = false;
        if (accepted) {
          window.rdAdminDirty = false;
          history.go(next - index);
        }
      });
      return;
    }
    index = next;
    window.rdAdminDirty = false;
  }, true);

  window.addEventListener('beforeunload', event => {
    if (window.rdAdminDirty) {
      event.preventDefault();
      event.returnValue = '';
    }
  });
  document.addEventListener('click', event => {
    const anchor = event.target.closest?.('a');
    if (!window.rdAdminDirty || !anchor || anchor.target === '_blank' || anchor.hasAttribute('download')
        || event.button !== 0 || event.ctrlKey || event.metaKey || event.shiftKey || event.altKey) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    if (navigationPending || confirmation) return;
    navigationPending = true;
    window.rdAdminConfirm('leave').then(accepted => {
      navigationPending = false;
      if (accepted) {
        window.rdAdminDirty = false;
        anchor.click();
      }
    });
  }, true);
  document.addEventListener('keydown', event => {
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 's') {
      const button = document.getElementById('admin-save');
      if (button) {
        event.preventDefault();
        button.click();
      }
    }
  });
})();
