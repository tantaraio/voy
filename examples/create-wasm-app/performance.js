export const perf = () => {
  const t0 = performance.now();
  return {
    stop() {
      const t1 = performance.now();
      const seconds = ((t1 - t0) / 1000).toFixed(2);
      return seconds;
    },
  };
};
