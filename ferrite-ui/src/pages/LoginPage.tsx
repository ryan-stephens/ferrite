import { createSignal, Show } from 'solid-js';
import { Flame, Eye, EyeOff } from 'lucide-solid';
import { login, authState } from '../stores/auth';
import { api } from '../api';

interface LoginPageProps {
  onSuccess: () => void;
}

export default function LoginPage(props: LoginPageProps) {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [confirmPassword, setConfirmPassword] = createSignal('');
  const [showPassword, setShowPassword] = createSignal(false);
  const [error, setError] = createSignal('');
  const [loading, setLoading] = createSignal(false);

  const isSetup = () => !authState().hasUsers;

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!username().trim() || !password().trim()) return;

    if (isSetup() && password() !== confirmPassword()) {
      setError('Passwords do not match');
      return;
    }
    if (isSetup() && password().length < 6) {
      setError('Password must be at least 6 characters');
      return;
    }

    setLoading(true);
    setError('');
    try {
      if (isSetup()) {
        await api.createUser(username(), password());
      }
      await login(username(), password());
      props.onSuccess();
    } catch (err: any) {
      setError(err.message || (isSetup() ? 'Account creation failed' : 'Login failed'));
    }
    setLoading(false);
  }

  return (
    <div class="min-h-screen flex items-center justify-center bg-surface p-4">
      <div class="w-full max-w-sm animate-scale-in">
        {/* Logo */}
        <div class="flex flex-col items-center mb-8">
          <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-ferrite-500 to-ferrite-700 flex items-center justify-center mb-4 shadow-lg shadow-ferrite-500/20">
            <Flame class="w-7 h-7 text-white" />
          </div>
          <h1 class="text-2xl font-bold text-white">Ferrite</h1>
          <Show when={isSetup()} fallback={
            <p class="text-sm text-surface-700 mt-1">Sign in to your media server</p>
          }>
            <p class="text-sm text-ferrite-400 mt-1">Welcome! Create your admin account</p>
          </Show>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} class="card p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1.5">Username</label>
            <input
              type="text"
              class="input-field"
              placeholder="Enter username"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
              autofocus
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1.5">Password</label>
            <div class="relative">
              <input
                type={showPassword() ? 'text' : 'password'}
                class="input-field pr-10"
                placeholder="Enter password"
                value={password()}
                onInput={(e) => setPassword(e.currentTarget.value)}
              />
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-surface-700 hover:text-gray-300 transition-colors"
                onClick={() => setShowPassword(!showPassword())}
              >
                {showPassword() ? <EyeOff class="w-4 h-4" /> : <Eye class="w-4 h-4" />}
              </button>
            </div>
          </div>

          <Show when={isSetup()}>
            <div>
              <label class="block text-sm font-medium text-gray-400 mb-1.5">Confirm Password</label>
              <input
                type={showPassword() ? 'text' : 'password'}
                class="input-field"
                placeholder="Confirm password"
                value={confirmPassword()}
                onInput={(e) => setConfirmPassword(e.currentTarget.value)}
              />
            </div>
          </Show>

          {error() && (
            <div class="px-3 py-2 rounded-lg bg-red-500/10 border border-red-500/20 text-sm text-red-400">
              {error()}
            </div>
          )}

          <button
            type="submit"
            class="btn-primary w-full"
            disabled={loading()}
          >
            {loading() ? (
              <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
            ) : isSetup() ? 'Create Account' : 'Sign In'}
          </button>
        </form>
      </div>
    </div>
  );
}
