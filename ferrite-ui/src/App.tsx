import { Show, onMount } from 'solid-js';
import { Router, Route } from '@solidjs/router';
import { Toaster } from 'solid-toast';
import { authState, showLogin, initAuth } from './stores/auth';
import { loadLibraries, loadMedia } from './stores/media';
import AppShell from './layout/AppShell';
import LoginPage from './pages/LoginPage';
import HomePage from './pages/HomePage';
import SearchPage from './pages/SearchPage';
import LibraryPage from './pages/LibraryPage';
import MediaDetailPage from './pages/MediaDetailPage';
import PlayerPage from './pages/PlayerPage';
import SettingsPage from './pages/SettingsPage';
import AdminPage from './pages/AdminPage';
import ShowsPage from './pages/ShowsPage';
import ShowDetailPage from './pages/ShowDetailPage';

export default function App() {
  onMount(async () => {
    await initAuth();
    if (authState().authenticated) {
      await loadLibraries();
      await loadMedia();
    }
  });

  return (
    <>
      <Toaster
        position="bottom-right"
        toastOptions={{
          style: {
            background: '#181824',
            color: '#e5e5e5',
            border: '1px solid rgba(255,255,255,0.05)',
            'border-radius': '0.75rem',
            'font-size': '0.875rem',
          },
        }}
      />

      <Show when={authState().loading}>
        <div class="min-h-screen flex items-center justify-center bg-surface">
          <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
        </div>
      </Show>

      <Show when={!authState().loading && showLogin()}>
        <LoginPage onSuccess={async () => {
          await loadLibraries();
          await loadMedia();
        }} />
      </Show>

      <Show when={!authState().loading && !showLogin()}>
        <Router>
          {/* Player is full-screen, no shell */}
          <Route path="/player/:id" component={PlayerPage} />

          {/* All other routes use the app shell */}
          <Route path="/" component={(props) => <AppShell>{props.children}</AppShell>}>
            <Route path="/" component={HomePage} />
            <Route path="/search" component={SearchPage} />
            <Route path="/library/:id" component={LibraryPage} />
            <Route path="/shows/library/:id" component={ShowsPage} />
            <Route path="/shows/:id" component={ShowDetailPage} />
            <Route path="/media/:id" component={MediaDetailPage} />
            <Route path="/continue" component={HomePage} />
            <Route path="/collections" component={HomePage} />
            <Route path="/settings" component={SettingsPage} />
            <Route path="/admin" component={AdminPage} />
          </Route>
        </Router>
      </Show>
    </>
  );
}
