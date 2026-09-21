import { useApiRequest } from "./use_api_request";
import type {
  AnalyzeCircuitResponse,
  EstimateCircuitResponse,
  QCTidyRequest,
  QCTidyResponse,
} from "~/types/api";

async function postJson<TResponse>(
  path: string,
  body: unknown,
): Promise<TResponse> {
  const response = await fetch(path, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    throw new Error(`API Error: ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as TResponse;
}

/** `POST /simplify` — expects `{ circuit, iterations }`, returns `{ circuit }`. */
function simplifyCircuit(request: QCTidyRequest): Promise<QCTidyResponse> {
  return postJson<QCTidyResponse>("/api/simplify", request);
}

/** `POST /estimate` — expects `{ circuit, shots? }`, returns `{ estimates }`. */
function estimateCircuit(
  circuit: QCTidyRequest["circuit"],
): Promise<EstimateCircuitResponse> {
  return postJson<EstimateCircuitResponse>("/api/estimate", { circuit });
}

/** `POST /analyze` — expects `{ circuit, mode? }`, returns `{ metrics }`. */
function analyzeCircuit(
  circuit: QCTidyRequest["circuit"],
): Promise<AnalyzeCircuitResponse> {
  return postJson<AnalyzeCircuitResponse>("/api/analyze", { circuit });
}

export function useSimplifyCircuit() {
  return useApiRequest(simplifyCircuit);
}

export function useEstimateCircuit() {
  return useApiRequest(estimateCircuit);
}

export function useAnalyzeCircuit() {
  return useApiRequest(analyzeCircuit);
}
