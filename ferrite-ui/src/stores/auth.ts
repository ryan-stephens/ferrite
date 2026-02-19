import { createSignal } from 'solid-js';
import { api, setToken as storeToken, clearToken as removeToken, getToken } from '../api';

export interface AuthState {
  authenticated: boolean;
  authRequired: boolean;
  hasUsers: boolean;
  loading: boolean;
}

const [authState, setAuthState] = createSignal<AuthState>({
  authenticated: false,
  authRequired: false,
  hasUsers: false,
  loading: true,
});

const [showLogin, setShowLogin] = createSignal(false);

/** Initialize auth — check server status and validate existing token. */
async function initAuth(): Promise<void> {
  try {
    const status = await api.authStatus();
    if (status.auth_required) {
      setAuthState(prev => ({ ...prev, authRequired: true, hasUsers: status.has_users }));
      if (!getToken()) {
        setShowLogin(true);
        setAuthState(prev => ({ ...prev, loading: false }));
        return;
      }
      try {
        await api.info();
        setAuthState(prev => ({ ...prev, authenticated: true, loading: false }));
      } catch {
        setShowLogin(true);
        setAuthState(prev => ({ ...prev, loading: false }));
      }
    } else {
      setAuthState(prev => ({ ...prev, authenticated: true, loading: false }));
    }
  } catch {
    setAuthState(prev => ({ ...prev, authenticated: true, loading: false }));
  }
}

async function login(username: string, password: string): Promise<void> {
  const data = await api.login(username, password);
  storeToken(data.token);
  setAuthState(prev => ({ ...prev, authenticated: true }));
  setShowLogin(false);
}

function logout(): void {
  removeToken();
  setAuthState(prev => ({ ...prev, authenticated: false }));
  setShowLogin(true);
}

// Listen for 401 events from API layer
window.addEventListener('ferrite:unauthorized', () => {
  setAuthState(prev => ({ ...prev, authenticated: false }));
  // Re-fetch auth status so hasUsers is correct before showing login
  api.authStatus().then(status => {
    setAuthState(prev => ({ ...prev, authRequired: status.auth_required, hasUsers: status.has_users }));
    setShowLogin(true);
  }).catch(() => {
    setShowLogin(true);
  });
});

export { authState, showLogin, setShowLogin, initAuth, login, logout };
