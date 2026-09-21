import { useCallback, useState } from "react";

export interface ApiRequest<TResult> {
  data: TResult | null;
  error: string | null;
  isLoading: boolean;
  reset: () => void;
  clearError: () => void;
}

/**
 * Wrap an async request with loading/error/data state.
 *
 * The returned `run` resolves with the result, or `null` when the request fails
 * (the message is exposed through `error`).
 */
export function useApiRequest<TArgs extends unknown[], TResult>(
  request: (...args: TArgs) => Promise<TResult>,
) {
  const [data, setData] = useState<TResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const run = useCallback(
    async (...args: TArgs): Promise<TResult | null> => {
      setIsLoading(true);
      setError(null);

      try {
        const result = await request(...args);
        setData(result);
        return result;
      } catch (caught) {
        setError(caught instanceof Error ? caught.message : "Request failed.");
        return null;
      } finally {
        setIsLoading(false);
      }
    },
    [request],
  );

  const reset = useCallback(() => {
    setData(null);
    setError(null);
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  return { data, error, isLoading, run, reset, clearError };
}
