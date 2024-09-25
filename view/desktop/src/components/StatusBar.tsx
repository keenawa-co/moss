import type { ComponentPropsWithoutRef } from "react";
import { twMerge } from "tailwind-merge";

const StatusBar = ({ branch, className }: { branch?: string } & ComponentPropsWithoutRef<"div">) => {
  return (
    <footer
      className={twMerge(
        "flex items-center justify-end bg-[rgba(var(--color-statusBar-background))] px-5 text-xs text-stone-50 z-100 [&>*:first-child]:mr-auto",
        className
      )}
    >
      <StatusBarButton className="group">
        <svg
          className="opacity-80 transition group-hover:opacity-100 group-focus:opacity-100"
          fill="none"
          height="18"
          viewBox="0 0 18 18"
          width="18"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g opacity=".8" stroke="white">
            <path d="m6 6 2 1.5L6 9" strokeLinejoin="round" />
            <path d="M8.4 12h3" strokeLinecap="round" />
            <rect height="11" rx="3.5" width="11" x="3.5" y="3.5" />
          </g>
        </svg>
        <span>Console</span>
      </StatusBarButton>

      {branch ? (
        <StatusBarButton className="group">
          <svg
            className="shrink-0 opacity-80 transition group-hover:opacity-100 group-focus:opacity-100"
            fill="none"
            height="18"
            viewBox="0 0 14 15"
            width="18"
            xmlns="http://www.w3.org/2000/svg"
          >
            <g opacity=".8">
              <path
                d="M3 13.42a.5.5 0 1 0 1 0H3Zm1 0v-8.4H3v8.4h1Zm-1-8.4V6.2h1V5.02H3ZM3 6.2c0 1.13.4 2.57 1.28 3.73A5.13 5.13 0 0 0 8.5 12v-1c-1.59 0-2.7-.72-3.42-1.68A5.41 5.41 0 0 1 4 6.2H3ZM8.5 12h.78v-1H8.5v1ZM4.23 3.8c0 .4-.33.72-.73.72v1c.95 0 1.73-.77 1.73-1.72h-1Zm-.73.72a.72.72 0 0 1-.72-.72h-1c0 .95.77 1.72 1.72 1.72v-1Zm-.72-.72c0-.4.32-.73.72-.73v-1c-.95 0-1.72.78-1.72 1.73h1Zm.72-.73c.4 0 .73.33.73.73h1c0-.95-.78-1.73-1.73-1.73v1Zm7.73 8.43c0 .4-.33.72-.73.72v1c.95 0 1.73-.77 1.73-1.72h-1Zm-.73.72a.73.73 0 0 1-.72-.72h-1c0 .95.77 1.72 1.72 1.72v-1Zm-.72-.72c0-.4.32-.73.72-.73v-1c-.95 0-1.72.78-1.72 1.73h1Zm.72-.73c.4 0 .73.33.73.73h1c0-.95-.78-1.73-1.73-1.73v1Z"
                fill="#fff"
                opacity=".9"
              />
            </g>
          </svg>
          <span className="line-clamp-1">{branch}</span>
        </StatusBarButton>
      ) : null}

      <StatusBarButton className="group">
        <svg
          className="h-4.5 w-4.5 opacity-80 transition group-hover:opacity-100 group-focus:opacity-100"
          fill="none"
          height="18"
          viewBox="0 0 14 14"
          width="18"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            clipRule="evenodd"
            d="M5.36 1.48a.53.53 0 0 0-.56.89c.26.17.48.41.69.7a.52.52 0 1 0 .85-.6 3.57 3.57 0 0 0-.98-.99Zm1.52 2.95-2.25.22.09.37.25.73.34.67.39.61.03.04.03-.05.43-.74.35-.79.28-.85.06-.2Zm1.11-.1 1.34-.13a.52.52 0 1 0-.1-1.05l-7.35.7a.52.52 0 1 0 .1 1.05l1.6-.15.13.57.3.84.38.78.46.71.23.28-.37.44-.64.61-.71.56-.8.53-.89.5a.52.52 0 1 0 .51.91l.92-.51.87-.58.79-.62.71-.68.34-.4.2.19.66.5.7.42-.22.46-.01.02-.78 1.56a.52.52 0 1 0 .94.47l.64-1.28h3.02l.65 1.28a.53.53 0 0 0 .94-.47l-.79-1.56v-.02L9.91 6.59a.52.52 0 0 0-.94 0L7.85 8.86l-.59-.35-.58-.44-.22-.2.2-.29.47-.82.4-.89.3-.93.16-.61Zm2.45 5.65H8.46l1-1.98.98 1.98Z"
            fill="#fff"
            fillRule="evenodd"
            opacity=".8"
          />
        </svg>
        <span>English</span>
      </StatusBarButton>

      <StatusBarButton className="group">
        <svg
          className="h-4.5 w-4.5 opacity-80 transition group-hover:opacity-100 group-focus:opacity-100"
          fill="none"
          height="18"
          viewBox="-4 -4 18 18"
          width="18"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            clipRule="evenodd"
            d="M6 1.1a4.9 4.9 0 1 0 0 9.8 4.9 4.9 0 0 0 0-9.8ZM0 6a6 6 0 1 1 12 0A6 6 0 0 1 0 6Zm5.3-2.3a.7.7 0 1 1 1.4 0 .7.7 0 0 1-1.4 0ZM6 5.5c.3 0 .5.2.5.5v2.5a.5.5 0 1 1-1 0V6c0-.3.2-.5.5-.5Z"
            fill="#fff"
            fillRule="evenodd"
          />
        </svg>
        <span>About</span>
      </StatusBarButton>
    </footer>
  );
};

export default StatusBar;

function StatusBarButton({ children, className }: ComponentPropsWithoutRef<"button">) {
  return (
    <button
      className={twMerge(
        "flex items-center gap-2 px-2.5 py-0.5 transition hover:bg-white hover:bg-opacity-10 focus:bg-white focus:bg-opacity-10",
        className
      )}
    >
      {children}
    </button>
  );
}
