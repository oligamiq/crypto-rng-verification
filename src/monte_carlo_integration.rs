use std::sync::Arc;

use crate::rng_trait::RNG;

pub struct MonteCarloIntegration {
    rng: Box<dyn RNG + Send + Sync>,
    target_function: Arc<Box<dyn Fn(f64) -> f64 + 'static + Send + Sync>>,
    sigma_function: Box<
        dyn Fn(Vec<f64>, Arc<Box<dyn Fn(f64) -> f64 + 'static + Send + Sync>>) -> f64 + Send + Sync,
    >,
    range: (f64, f64),
    answer: f64,
}

impl MonteCarloIntegration {
    pub fn new<R, T, S>(
        rng: R,
        target_func: T,
        sigma_func: S,
        range: (f64, f64),
        answer: f64,
    ) -> Self
    where
        R: RNG + 'static + Send + Sync,
        T: Fn(f64) -> f64 + 'static + Send + Sync,
        S: Fn(Vec<f64>, Arc<Box<dyn Fn(f64) -> f64 + Send + Sync>>) -> f64 + 'static + Send + Sync,
    {
        let rng = Box::new(rng);
        let target_function: Arc<Box<dyn Fn(f64) -> f64 + 'static + Send + Sync>> =
            Arc::new(Box::new(target_func));
        let sigma_function = Box::new(sigma_func);
        Self {
            rng,
            target_function,
            sigma_function,
            range,
            answer,
        }
    }

    /// 4
    /// ∫√(2x+1) dx = 26/3
    /// 0
    pub fn template_new<R>(rng: R) -> Self
    where
        R: RNG + 'static + Send + Sync,
    {
        Self::new(
            rng,
            |x| ((2f64 * x + 1f64) as f64).sqrt(),
            |p, f: Arc<Box<dyn Fn(f64) -> f64 + Send + Sync>>| p.iter().map(|&x| f(x)).sum::<f64>(),
            (0.0, 4.0),
            26.0 / 3.0,
        )
    }

    // Nサンプル発生させて積分を近似計算し、真値との誤差を計算（積分の真値は26.0/3.0）
    pub fn err(&mut self, n: usize) -> f64 {
        let p: Vec<f64> = (0..n)
            .map(|_| self.rng.gen_range(self.range.0..self.range.1))
            .collect();
        let approx_integral: f64 = (self.range.1 - self.range.0)
            * (self.sigma_function)(p, self.target_function.clone())
            / n as f64;
        (approx_integral - self.answer).abs()
    }
}
