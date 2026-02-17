interface StatusBarProps {
  status: string;
  scanning: boolean;
}

export default function StatusBar(props: StatusBarProps) {
  return (
    <div class={`fixed bottom-0 left-0 right-0 bg-surface-100 border-t border-surface-300 px-6 py-2 text-sm ${
      props.scanning ? 'text-ferrite-500' : 'text-gray-500'
    }`}>
      {props.scanning && (
        <span class="inline-block w-2 h-2 bg-ferrite-500 rounded-full mr-2 animate-pulse" />
      )}
      {props.status}
    </div>
  );
}
