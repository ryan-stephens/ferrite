import { createSignal } from 'solid-js';

interface LoginProps {
  onLogin: (username: string, password: string) => Promise<void>;
}

export default function Login(props: LoginProps) {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [error, setError] = createSignal('');
  const [loading, setLoading] = createSignal(false);

  async function handleSubmit() {
    if (!username().trim() || !password()) return;
    setLoading(true);
    setError('');
    try {
      await props.onLogin(username().trim(), password());
    } catch (e: any) {
      setError(e.message || 'Invalid credentials');
    }
    setLoading(false);
  }

  return (
    <div class="fixed inset-0 bg-surface z-[300] flex items-center justify-center">
      <div class="bg-surface-100 border border-surface-300 rounded-xl p-10 w-[360px] text-center">
        <h1 class="text-ferrite-500 text-2xl font-bold mb-2">Ferrite</h1>
        <p class="text-gray-500 mb-6">Sign in to continue</p>

        {error() && (
          <div class="text-red-500 text-sm mb-4">{error()}</div>
        )}

        <label class="block text-left text-sm text-gray-400 mb-1">Username</label>
        <input
          type="text"
          class="w-full bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 mb-4 focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          placeholder="Username"
          value={username()}
          onInput={e => setUsername(e.currentTarget.value)}
          onKeyDown={e => e.key === 'Enter' && document.getElementById('login-pw')?.focus()}
          autofocus
        />

        <label class="block text-left text-sm text-gray-400 mb-1">Password</label>
        <input
          id="login-pw"
          type="password"
          class="w-full bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 mb-4 focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          placeholder="Password"
          value={password()}
          onInput={e => setPassword(e.currentTarget.value)}
          onKeyDown={e => e.key === 'Enter' && handleSubmit()}
        />

        <button
          class="w-full bg-ferrite-500 hover:bg-ferrite-600 text-white font-medium py-2.5 rounded-md transition-colors disabled:opacity-50"
          onClick={handleSubmit}
          disabled={loading()}
        >
          {loading() ? 'Signing in...' : 'Sign In'}
        </button>
      </div>
    </div>
  );
}
