import { onMount, onCleanup } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { collapsed } from './Sidebar';
import Sidebar from './Sidebar';
import TopBar from './TopBar';

interface AppShellProps {
  children: any;
}

export default function AppShell(props: AppShellProps) {
  const navigate = useNavigate();

  function handleGlobalKeyDown(e: KeyboardEvent) {
    const isInput = ['INPUT', 'SELECT', 'TEXTAREA'].includes(
      (document.activeElement?.tagName || ''),
    );

    // Ctrl+K or / to focus search
    if ((e.ctrlKey && e.key === 'k') || (!isInput && e.key === '/')) {
      e.preventDefault();
      navigate('/search');
    }
  }

  onMount(() => document.addEventListener('keydown', handleGlobalKeyDown));
  onCleanup(() => document.removeEventListener('keydown', handleGlobalKeyDown));

  return (
    <div class="min-h-screen bg-surface">
      <Sidebar />
      <div
        class={`transition-all duration-300 ease-out ${collapsed() ? 'ml-sidebar-collapsed' : 'ml-sidebar'}`}
      >
        <TopBar />
        <main class="min-h-[calc(100vh-4rem)]">
          {props.children}
        </main>
      </div>
    </div>
  );
}
