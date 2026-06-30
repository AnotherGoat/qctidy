const ABSOLUTE_TOLERANCE = 1e-8;
const RELATIVE_TOLERANCE = 1e-5;
const COMMON_PI_DENOMINATORS = [1, 2, 3, 4, 6, 8, 12];

export const formatAngle = (num: number): string => {
  if (Math.abs(num) < ABSOLUTE_TOLERANCE) {
    return "0";
  }

  const piRatio = num / Math.PI;

  for (const denominator of COMMON_PI_DENOMINATORS) {
    const piMultiple = formatPiMultiple(piRatio, denominator);

    if (piMultiple) {
      return piMultiple;
    }
  }

  return num.toFixed(2);
};

const formatPiMultiple = (
  piRatio: number,
  denominator: number,
): string | null => {
  const scaled = piRatio * denominator;
  const rounded = Math.round(scaled);

  if (isClose(scaled, rounded)) {
    if (denominator === 1) {
      return `${rounded === 1 ? "" : rounded === -1 ? "-" : rounded}π`;
    } else {
      return `${rounded === 1 ? "" : rounded === -1 ? "-" : rounded}π/${denominator}`;
    }
  }

  return null;
};

const isClose = (a: number, b: number): boolean => {
  return (
    Math.abs(a - b) <=
    ABSOLUTE_TOLERANCE + RELATIVE_TOLERANCE * Math.max(Math.abs(a), Math.abs(b))
  );
};
