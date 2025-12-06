use glam::Vec3;

// Aizawa attractor parameters
#[derive(Clone, Copy, Debug)]
pub struct AizawaParams {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Default for AizawaParams {
    fn default() -> Self {
        Self {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1,
        }
    }
}

// Calculate the derivative at a given point
// dx/dt = (z - b)*x - d*y
// dy/dt = d*x + (z - b)*y  
// dz/dt = c + a*z - z³/3 - (x² + y²)*(1 + e*z) + f*z*x³
pub fn derivative(p: Vec3, params: &AizawaParams) -> Vec3 {
    let AizawaParams{a, b, c, d, e, f} = *params;
    let (x, y, z) = (p.x, p.y, p.z);

    Vec3::new(
        (z - b)*x - d*y,
        d*x + (z - b)*y,
        c + a*z - z.powi(3)/3.0
            - (x*x + y*y)*(1.0 + e*z)
            + f*z*x.powi(3),
    )
}

// Integrate one step using Euler's method
#[allow(dead_code)]
pub fn step_euler(p: Vec3, params: &AizawaParams, dt: f32) -> Vec3 {
    p + derivative(p, params)*dt
}

// Heun's method (or modified Euler's method)
#[allow(dead_code)]
pub fn step_heun(p: Vec3, params: &AizawaParams, dt: f32) -> Vec3 {
    let p_next = step_euler(p, params, dt);

    let k1 = derivative(p, params);
    let k2 = derivative(p_next, params);

    p + (k1 + k2)/2.0 * dt
}

// Runge-Kutta method of 4th order
#[allow(dead_code)]
pub fn step_rk4(p: Vec3, params: &AizawaParams, dt: f32) -> Vec3 {
    let k1 = derivative(p, params);
    let k2 = derivative(p + 0.5*k1*dt, params);
    let k3 = derivative(p + 0.5*k2*dt, params);
    let k4 = derivative(p + k3*dt, params);

    p + (k1 + 2.0*k2 + 2.0*k3 + k4)/6.0 * dt
}
