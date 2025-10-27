declare module '*/achronyme-core.mjs' {
  interface AchronymeCoreModule {
    eval(expression: string): string;
    reset(): void;
  }

  export default function createAchronymeModule(): Promise<AchronymeCoreModule>;
}
