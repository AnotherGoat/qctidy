export const formatAngle = (num: number): string => {
  // If it's a decimal, round to 2 decimal places to keep it clean
  const rounded = Math.round(num * 100) / 100;
  return `${rounded}°`;
};
