export default function InteractionPanel({ children, className = "" }) {
  return (
    <div
      className={`
        flex-1
        h-full
        min-h-0
        min-w-0
        ${className}
      `}
    >
      {children}
    </div>
  );
}
