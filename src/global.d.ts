declare namespace svelteHTML {
  interface HTMLAttributes<T> {
    'on:resizing'?: (event: CustomEvent<boolean>) => void;
  }
}
