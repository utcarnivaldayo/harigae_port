extern crate wasm_bindgen;
//extern crate console_error_panic_hook;
//use std::panic;
use num::complex::Complex64;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

struct SearchPoint {
    position: Vec<f64>,
    mybest: Vec<f64>,
    delta: Vec<f64>,
    evaluation: f64,
    violation: f64,
    epsilon: f64,
}

impl SearchPoint {
    fn new(dimention: usize) -> SearchPoint {
        let mut search_point = SearchPoint {
            position: Vec::with_capacity(dimention),
            mybest: Vec::with_capacity(dimention),
            delta: Vec::with_capacity(dimention),
            evaluation: std::f64::MAX,
            violation: std::f64::MAX,
            epsilon: 0.0,
        };

        for _i in 0..dimention {
            search_point.position.push(0.0);
            search_point.mybest.push(0.0);
            search_point.delta.push(0.0);
        }
        search_point
    }

    fn compair_epsilon(&self, cmp: &SearchPoint) -> bool {
        if self.violation < self.epsilon && cmp.violation < self.epsilon {
            if self.evaluation < cmp.evaluation {
                true
            } else {
                false
            }
        } else if self.violation == 0.0 && cmp.violation == 0.0 {
            if self.evaluation < cmp.evaluation {
                true
            } else {
                false
            }
        } else {
            if self.violation < cmp.violation {
                true
            } else {
                false
            }
        }
    }
}

struct Iir {
    scale: f64,
    a: Vec<f64>,
    b: Vec<f64>,
    normalized_angular_frequency: Vec<f64>,
    complex_sin: Vec<Complex64>,
    complex_sin2: Vec<Complex64>,
    response: Vec<Complex64>,
    group_delay: Vec<f64>,
}

impl Iir {
    fn new() -> Iir {
        Iir {
            scale: 0.0,
            a: Vec::new(),
            b: Vec::new(),
            normalized_angular_frequency: Vec::new(),
            complex_sin: Vec::new(),
            complex_sin2: Vec::new(),
            response: Vec::new(),
            group_delay: Vec::new(),
        }
    }

    fn create_all_band(&mut self, division: usize) {
        let delta = PI / (division - 1) as f64;

        for i in 0..division {
            self.normalized_angular_frequency.push(delta * i as f64);
            self.response.push(Complex64::new(0.0, 0.0));
            self.group_delay.push(0.0);
            self.complex_sin.push(Complex64::new(
                self.normalized_angular_frequency[i].cos(),
                -self.normalized_angular_frequency[i].sin(),
            ));
            self.complex_sin2.push(Complex64::new(
                (2.0 * self.normalized_angular_frequency[i]).cos(),
                -(2.0 * self.normalized_angular_frequency[i]).sin(),
            ));
        }
    }

    fn create_twoband(
        &mut self,
        pass_band_edge: f64,
        stop_band_edge: f64,
        division_approximation_band: usize,
        division_transition_band: usize,
    ) -> (usize, usize) {
        let division_pass_band: usize = (pass_band_edge / (PI - (stop_band_edge - pass_band_edge))
            * division_approximation_band as f64) as usize
            + 1usize;

        let delta_pass_band = pass_band_edge / (division_pass_band - 1usize) as f64;

        let division_stop_band: usize = division_approximation_band - division_pass_band;

        let delta_stop_band = (PI - stop_band_edge) / (division_stop_band - 1usize) as f64;

        let delta_transition =
            (stop_band_edge - pass_band_edge) / (division_transition_band - 1usize) as f64;

        //pass band
        for i in 0..division_pass_band {
            self.normalized_angular_frequency
                .push(delta_pass_band * i as f64);
        }

        //transition band
        self.normalized_angular_frequency
            .push(pass_band_edge + std::f64::EPSILON);
        for i in 1..(division_transition_band - 1) {
            self.normalized_angular_frequency
                .push(delta_transition * i as f64 + pass_band_edge);
        }
        self.normalized_angular_frequency
            .push(stop_band_edge - std::f64::EPSILON);
        //stop band
        for i in 0..division_stop_band {
            self.normalized_angular_frequency
                .push(delta_stop_band * i as f64 + stop_band_edge);
        }

        //create response gd
        for i in 0..self.normalized_angular_frequency.len() {
            self.response.push(Complex64::new(0.0, 0.0));
            self.group_delay.push(0.0);
            self.complex_sin.push(Complex64::new(
                self.normalized_angular_frequency[i].cos(),
                -self.normalized_angular_frequency[i].sin(),
            ));
            self.complex_sin2.push(Complex64::new(
                (2.0 * self.normalized_angular_frequency[i]).cos(),
                -(2.0 * self.normalized_angular_frequency[i]).sin(),
            ));
        }

        (
            division_pass_band,
            division_pass_band + division_transition_band - 1,
        )
    }

    fn calculate_frequency_response(&mut self) {
        for i in 0..self.normalized_angular_frequency.len() {
            self.response[i] = Complex64::new(self.scale, 0.0);
            for j in 0..(self.a.len() >> 1) {
                self.response[i] *= 1.0
                    + self.a[j << 1] * self.complex_sin[i]
                    + self.a[(j << 1) + 1] * self.complex_sin2[i];
            }

            for j in 0..(self.b.len() >> 1) {
                self.response[i] /= 1.0
                    + self.b[j << 1] * self.complex_sin[i]
                    + self.b[(j << 1) + 1] * self.complex_sin2[i];
            }
        }

        if self.a.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                self.response[i] *= 1.0 + self.a[self.a.len() - 1] * self.complex_sin[i];
            }
        }

        if self.b.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                self.response[i] /= 1.0 + self.b[self.b.len() - 1] * self.complex_sin[i];
            }
        }
    }

    fn calculate_frequency_response_reference(&mut self, scale: f64, a: &[f64], b: &[f64]) {
        for i in 0..self.normalized_angular_frequency.len() {
            self.response[i] = Complex64::new(scale, 0.0);
            for j in 0..(a.len() >> 1) {
                self.response[i] *=
                    1.0 + a[j << 1] * self.complex_sin[i] + a[(j << 1) + 1] * self.complex_sin2[i];
            }

            for j in 0..(b.len() >> 1) {
                self.response[i] /=
                    1.0 + b[j << 1] * self.complex_sin[i] + b[(j << 1) + 1] * self.complex_sin2[i];
            }
        }

        if a.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                self.response[i] *= 1.0 + a[a.len() - 1] * self.complex_sin[i];
            }
        }

        if b.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                self.response[i] /= 1.0 + b[b.len() - 1] * self.complex_sin[i];
            }
        }
    }

    fn calculate_group_delay(&mut self) {
        let mut buffer_prime = Complex64::new(0.0, 0.0);
        let mut buffer = Complex64::new(0.0, 0.0);
        for i in 0..self.normalized_angular_frequency.len() {
            for j in 0..(self.a.len() >> 1) {
                buffer_prime = self.a[j << 1] * self.complex_sin[i]
                    + 2.0 * self.a[(j << 1) + 1] * self.complex_sin2[i];
                buffer_prime /= 1.0
                    + self.a[j << 1] * self.complex_sin[i]
                    + self.a[(j << 1) + 1] * self.complex_sin2[i];
                buffer += buffer_prime;
            }
            self.group_delay[i] = buffer.re;

            buffer = Complex64::new(0.0, 0.0);
            for j in 0..(self.b.len() >> 1) {
                buffer_prime = self.b[j << 1] * self.complex_sin[i]
                    + 2.0 * self.b[(j << 1) + 1] * self.complex_sin2[i];
                buffer_prime /= 1.0
                    + self.b[j << 1] * self.complex_sin[i]
                    + self.b[(j << 1) + 1] * self.complex_sin2[i];
                buffer += buffer_prime;
            }
            self.group_delay[i] -= buffer.re;
        }

        if self.a.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                buffer_prime = self.a[self.a.len() - 1] * self.complex_sin[i];
                buffer_prime /= 1.0 + self.a[self.a.len() - 1] * self.complex_sin[i];
                self.group_delay[i] += buffer_prime.re;
            }
        }

        if self.b.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                buffer_prime = self.b[self.b.len() - 1] * self.complex_sin[i];
                buffer_prime /= 1.0 + self.b[self.b.len() - 1] * self.complex_sin[i];
                self.group_delay[i] -= buffer_prime.re;
            }
        }
    }

    fn calculate_group_delay_reference(&mut self, a: &[f64], b: &[f64]) {
        let mut buffer_prime = Complex64::new(0.0, 0.0);
        let mut buffer = Complex64::new(0.0, 0.0);
        for i in 0..self.normalized_angular_frequency.len() {
            for j in 0..(a.len() >> 1) {
                buffer_prime =
                    a[j << 1] * self.complex_sin[i] + 2.0 * a[(j << 1) + 1] * self.complex_sin2[i];
                buffer_prime /=
                    1.0 + a[j << 1] * self.complex_sin[i] + a[(j << 1) + 1] * self.complex_sin2[i];
                buffer += buffer_prime;
            }
            self.group_delay[i] = buffer.re;

            buffer = Complex64::new(0.0, 0.0);
            for j in 0..(b.len() >> 1) {
                buffer_prime = b[j << 1] * self.complex_sin[i]
                    + 2.0 * b[(j << 1) + 1] * self.complex_sin2[i];
                buffer_prime /= 1.0
                    + b[j << 1] * self.complex_sin[i]
                    + b[(j << 1) + 1] * self.complex_sin2[i];
                buffer += buffer_prime;
            }
            self.group_delay[i] -= buffer.re;
        }

        if self.a.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                buffer_prime = self.a[self.a.len() - 1] * self.complex_sin[i];
                buffer_prime /= 1.0 + self.a[self.a.len() - 1] * self.complex_sin[i];
                self.group_delay[i] += buffer_prime.re;
            }
        }

        if self.b.len() % 2 == 1 {
            for i in 0..self.normalized_angular_frequency.len() {
                buffer_prime = self.b[self.b.len() - 1] * self.complex_sin[i];
                buffer_prime /= 1.0 + self.b[self.b.len() - 1] * self.complex_sin[i];
                self.group_delay[i] -= buffer_prime.re;
            }
        }
    }
    /*
    #[cfg(feature = "local")]
    fn write_magnitude(&self, filepath: &str) {
        let mut writer = csv::Writer::from_path(filepath).expect("D");

        for i in 0..self.normalized_angular_frequency.len() {
            let magnitude = self.response[i].norm();
            let db = 20.0 * magnitude.log10();
            writer
                .write_record(&[
                    self.normalized_angular_frequency[i].to_string(),
                    db.to_string(),
                ])
                .expect("Write Error");
        }
        writer.flush().expect("Failed flush");
    }
    #[cfg(feature = "local")]
    fn write_group_delay(&self, filepath: &str) {
        let mut writer = csv::Writer::from_path(filepath).expect("D");

        for i in 0..self.normalized_angular_frequency.len() {
            writer
                .write_record(&[
                    self.normalized_angular_frequency[i].to_string(),
                    self.group_delay[i].to_string(),
                ])
                .expect("Write Error");
        }
        writer.flush().expect("Failed flush");
    }
    */
    fn is_stability_for_second_order(&self, b1: f64, b2: f64) -> bool {
        if b2.abs() < 1.0 && b2 > (b1.abs() - 1.0) {
            true
        } else {
            false
        }
    }

    fn is_stability_for_first_order(&self, b1: f64) -> bool {
        if b1.abs() < 1.0 {
            true
        } else {
            false
        }
    }

    fn quadratic_formula(&self, c_1: f64, c_2: f64) -> Vec<Complex64> {
        let d: f64 = c_1 * c_1 - 4.0 * c_2;
        let mut roots: Vec<Complex64> = Vec::with_capacity(2);

        if d < 0.0 {
            let real = -c_1 / 2.0;
            let imag = (-d).sqrt() / 2.0;
            roots.push(Complex64::new(real, imag));
            roots.push(Complex64::new(real, imag));
        } else if d > 0.0 {
            if c_1 > 0.0 {
                roots.push(Complex64::new((-c_1 - d.sqrt()) / 2.0, 0.0));
            } else {
                roots.push(Complex64::new((-c_1 + d.sqrt()) / 2.0, 0.0));
            }
            roots.push(Complex64::new(c_2 / roots[0].re, 0.0));
        } else {
            roots.push(Complex64::new(-c_1 / 2.0, 0.0));
            roots.push(Complex64::new(-c_1 / 2.0, 0.0));
        }
        roots
    }
}

struct Pso {
    search_points: Vec<SearchPoint>,
    random: ThreadRng,
    group_best: SearchPoint,
    buffer_solution: SearchPoint,
    updating_curve: Vec<f64>,
    c1: f64,
    c2: f64,
    weight: f64,
}

impl Pso {
    fn new(number_of_search_points: usize, dimention: usize, weight: f64, c1: f64, c2: f64) -> Pso {
        let mut pso = Pso {
            search_points: Vec::with_capacity(number_of_search_points),
            random: rand::thread_rng(),
            group_best: SearchPoint::new(dimention),
            buffer_solution: SearchPoint::new(dimention),
            updating_curve: Vec::new(),
            c1: c1,
            c2: c2,
            weight: weight,
        };

        for _i in 0..number_of_search_points {
            pso.search_points.push(SearchPoint::new(dimention));
        }
        pso
    }
    fn update_position_velocity(&mut self) {
        for i in 0..self.search_points.len() {
            for j in 0..self.search_points[0].position.len() {
                self.search_points[i].delta[j] = self.weight * self.search_points[i].delta[j]
                    + self.c1
                        * self.random.gen::<f64>()
                        * (self.search_points[i].mybest[j] - self.search_points[i].mybest[j])
                    + self.c2
                        * self.random.gen::<f64>()
                        * (self.group_best.position[j] - self.search_points[i].position[j]);
                self.search_points[i].position[j] += self.search_points[i].delta[j];
            }
        }
    }

    fn update_evaluation(&mut self, iir_design: &mut IirDesign) {
        for i in 0..self.search_points.len() {
            iir_design.resampling_method(&mut self.search_points[i].position);
            self.buffer_solution.evaluation =
                iir_design.objective_function(&self.search_points[i].position);
            self.buffer_solution.violation = iir_design.violation_function();
            if !self.search_points[i].compair_epsilon(&self.buffer_solution) {
                self.search_points[i].evaluation = self.buffer_solution.evaluation;
                self.search_points[i].violation = self.buffer_solution.violation;
                for k in 0..self.search_points[i].position.len() {
                    self.search_points[i].mybest[k] = self.search_points[i].position[k];
                }

                if !self.group_best.compair_epsilon(&self.search_points[i]) {
                    self.group_best.evaluation = self.search_points[i].evaluation;
                    self.group_best.violation = self.search_points[i].violation;
                    for k in 0..self.search_points[i].position.len() {
                        self.group_best.position[k] = self.search_points[i].mybest[k];
                    }
                }
            }
        }
    }
}

struct IirDesign {
    iir: Iir,
    numerator_order: usize,
    denominator_order: usize,
    desired_group_delay: f64,
    max_ripple: f64,
    desired_response: Vec<Complex64>,
    pass_band_edge: f64,
    stop_band_edge: f64,
    division_approximation_band: usize,
    division_transition_band: usize,
    index_pass_band_edge: usize,
    index_stop_band_edge: usize,
}

impl IirDesign {
    fn new(
        numerator_order: usize,
        denominator_order: usize,
        desired_group_delay: f64,
        max_ripple: f64,
        pass_band_edge: f64,
        stop_band_edge: f64,
        division_approximation_band: usize,
        division_transition_band: usize,
    ) -> IirDesign {
        let mut iir_design = IirDesign {
            iir: Iir::new(),
            numerator_order: numerator_order,
            denominator_order: denominator_order,
            desired_group_delay: desired_group_delay,
            max_ripple: max_ripple,
            desired_response: Vec::new(),
            pass_band_edge: pass_band_edge,
            stop_band_edge: stop_band_edge,
            division_approximation_band: division_approximation_band,
            division_transition_band: division_transition_band,
            index_pass_band_edge: 0,
            index_stop_band_edge: 0,
        };
        let index = iir_design.iir.create_twoband(
            pass_band_edge,
            stop_band_edge,
            division_approximation_band,
            division_transition_band,
        );
        iir_design.index_pass_band_edge = index.0;
        iir_design.index_stop_band_edge = index.1;

        //desired response
        for i in 0..iir_design.index_pass_band_edge {
            iir_design.desired_response.push(Complex64::new(
                (desired_group_delay * iir_design.iir.normalized_angular_frequency[i]).cos(),
                -(desired_group_delay * iir_design.iir.normalized_angular_frequency[i]).sin(),
            ));
        }

        for _i in iir_design.index_pass_band_edge..iir_design.iir.normalized_angular_frequency.len()
        {
            iir_design.desired_response.push(Complex64::new(0.0, 0.0));
        }
        iir_design
    }

    fn objective_function(&mut self, x: &Vec<f64>) -> f64 {
        self.iir.calculate_frequency_response_reference(
            x[0].abs(),
            &x[1..(self.numerator_order + 1)],
            &x[(self.numerator_order + 1)..x.len()],
        );

        //pass
        let mut max_error = (self.desired_response[0] - self.iir.response[0]).norm();
        for i in 1..self.index_pass_band_edge {
            let buffer_error = (self.desired_response[i] - self.iir.response[i]).norm();
            if buffer_error > max_error {
                max_error = buffer_error;
            }
        }
        //println!("fp:{}", self.index_pass_band_edge);
        //println!("fs:{}", self.index_stop_band_edge);
        //stop
        for i in self.index_stop_band_edge..self.iir.normalized_angular_frequency.len() {
            let buffer_error = (self.desired_response[i] - self.iir.response[i]).norm();
            if buffer_error > max_error {
                max_error = buffer_error;
            }
        }
        max_error
    }

    fn violation_function(&self) -> f64 {
        let mut violation_value = self.iir.response[self.index_pass_band_edge + 1].norm();
        for i in (self.index_pass_band_edge + 2)..(self.index_stop_band_edge - 1) {
            let buffer = self.iir.response[i].norm();
            if violation_value < buffer {
                violation_value = buffer;
            }
        }
        if violation_value > self.max_ripple {
            violation_value - self.max_ripple
        } else {
            0.0
        }
    }

    fn resampling_method(&self, x: &mut Vec<f64>) {
        let offset: usize = self.numerator_order + 1;
        for i in 0..(self.denominator_order >> 1) {
            if !self
                .iir
                .is_stability_for_second_order(x[(i << 1) + offset], x[(i << 1) + offset + 1])
            {
                if x[(i << 1) + offset] * x[(i << 1) + offset] - 4.0 * x[(i << 1) + offset + 1]
                    < 0.0
                {
                    x[(i << 1) + offset] /= x[(i << 1) + offset + 1];
                    x[0] /= x[(i << 1) + offset + 1].abs();
                    x[(i << 1) + offset + 1] = 1.0 / x[(i << 1) + offset + 1];
                } else {
                    let roots = self
                        .iir
                        .quadratic_formula(x[(i << 1) + offset], x[(i << 1) + offset + 1]);
                    let flag_alpha: bool = self.iir.is_stability_for_first_order(roots[0].re);
                    let flag_beta: bool = self.iir.is_stability_for_first_order(roots[1].re);

                    if !flag_alpha && !flag_beta {
                        x[(i << 1) + offset] /= x[(i << 1) + offset + 1];
                        x[0] /= x[(i << 1) + offset + 1].abs();
                        x[(i << 1) + offset + 1] = 1.0 / x[(i << 1) + offset + 1];
                    } else if !flag_alpha {
                        x[0] /= roots[0].re.abs();
                        x[(i << 1) + offset] = -(1.0 / roots[0].re + roots[1].re);
                        x[(i << 1) + offset + 1] = roots[1].re / roots[0].re;
                    } else {
                        x[0] /= roots[1].re.abs();
                        x[(i << 1) + offset] = -(1.0 / roots[1].re + roots[0].re);
                        x[(i << 1) + offset + 1] = roots[0].re / roots[1].re;
                    }
                }
            }
        }

        if self.denominator_order % 2 == 1 {
            if !self.iir.is_stability_for_first_order(x[x.len() - 1]) {
                x[0] /= x[self.numerator_order + self.denominator_order];
                x[self.numerator_order + self.denominator_order] =
                    1.0 / x[self.numerator_order + self.denominator_order];
            }
        }
    }

    fn initialize_search_points(&self, search_points: &mut Vec<SearchPoint>, scale: f64, a: f64) {
        let mut rnd = rand::thread_rng();
        let offset = self.numerator_order + 1;
        for i in 0..search_points.len() {
            search_points[i].position[0] = rnd.gen::<f64>() * 2.0 * scale - scale;
            for j in 1..(self.numerator_order + 1) {
                search_points[i].position[j] = rnd.gen::<f64>() * 2.0 * a - a;
            }
            for j in 0..(self.denominator_order >> 1) {
                search_points[i].position[(j << 1) + offset + 1] = rnd.gen::<f64>() * 2.0 - 1.0;
                search_points[i].position[(j << 1) + offset] =
                    search_points[i].position[(j << 1) + offset + 1] + 1.0;
                search_points[i].position[(j << 1) + offset] =
                    rnd.gen::<f64>() * 2.0 * search_points[i].position[(j << 1) + offset]
                        - search_points[i].position[(j << 1) + offset];
            }

            if self.denominator_order % 2 == 1 {
                search_points[i].position[self.numerator_order + self.denominator_order] =
                    rnd.gen::<f64>() * 2.0 - 1.0;
            }
        }
    }
}
#[wasm_bindgen]
pub fn iir_design_pso(
    numerator_order: usize,
    denominator_order: usize,
    pass_band_edge: f64,
    stop_band_edge: f64,
    desired_group_delay: f64,
    max_ripple: f64,
    division_approximation_band: usize,
    division_transition_band: usize,
    number_of_search_points: usize,
    max_iteration: usize,
    weight: f64,
    c1: f64,
    c2: f64,
    init_scale: f64,
    init_a: f64,
    normalized_angular_frequency: &mut [f64],
    magnitude_response: &mut [f64],
    group_delay: &mut [f64],
    a: &mut [f64],
    b: &mut [f64],
) -> f64 {

    //panic::set_hook(Box::new(console_error_panic_hook::hook));

    let div = 1001;

    //init
    let mut iir_design = IirDesign::new(
        numerator_order,
        denominator_order,
        desired_group_delay,
        max_ripple,
        pass_band_edge * PI,
        stop_band_edge * PI,
        division_approximation_band,
        division_transition_band,
    );

    let mut pso = Pso::new(
        number_of_search_points,
        1 + numerator_order + denominator_order,
        weight,
        c1,
        c2,
    );
    iir_design.initialize_search_points(&mut pso.search_points, init_scale, init_a);
    pso.update_evaluation(&mut iir_design);

    //iteration
    for i in 0..max_iteration {
        pso.update_position_velocity();
        pso.update_evaluation(&mut iir_design);
    }
    let mut out = Iir::new();
    out.create_all_band(div);
    out.calculate_frequency_response_reference(
        pso.group_best.position[0].abs(),
        &pso.group_best.position[1..(iir_design.numerator_order + 1)],
        &pso.group_best.position[(iir_design.numerator_order + 1)..],
    );
    out.calculate_group_delay_reference(
        &pso.group_best.position[1..(iir_design.numerator_order + 1)],
        &pso.group_best.position[(iir_design.numerator_order + 1)..],
    );

    //copy response
    for i in 0..div {
        normalized_angular_frequency[i] = out.normalized_angular_frequency[i];
        magnitude_response[i] = 20.0 * (out.response[i].norm()).log10();
        group_delay[i] = out.group_delay[i];
    }

    //coef
    a[0] = pso.group_best.position[0].abs();
    for i in 1..(numerator_order + 1) {
        a[i] = pso.group_best.position[i];
    }

    for i in (numerator_order + 1)..pso.group_best.position.len() {
        b[i - (numerator_order + 1)] = pso.group_best.position[i];
    }

    pso.group_best.evaluation
}
/*
fn main() {
    /*
    let scale = 0.035065127;
    let a = vec![
        1.380465073,
        0.906419015,
        -4.005962557,
        5.1422312,
        0.116217525,
        0.971652724,
    ];
    let b = vec![-0.363082563, 0.793872877, -0.980290458, 0.366042632];

    let mut iir = Iir::new();
    iir.scale = scale;
    iir.a = a;
    iir.b = b;
    iir.create_all_band(1001);
    iir.calculate_frequency_response();
    iir.calculate_group_delay();
    //iir.write_magnitude("mag.csv");
    //iir.write_group_delay("gd.csv");
    */
    let mut iir_design = IirDesign::new(12, 8, 10.0, 1.0, 0.4 * PI, 0.5 * PI, 200, 50);
    let mut pso = Pso::new(100, 21, 0.85, 1.75, 1.75);
    iir_design.initialize_search_points(&mut pso.search_points, 0.5, 3.0);
    pso.update_evaluation(&mut iir_design);

    for i in 0..5000 {
        pso.update_position_velocity();
        pso.update_evaluation(&mut iir_design);
    }
    let mut out = Iir::new();
    out.create_all_band(1001);
    out.calculate_frequency_response_reference(
        pso.group_best.position[0],
        &pso.group_best.position[1..(iir_design.numerator_order + 1)],
        &pso.group_best.position[(iir_design.numerator_order + 1)..],
    );
    out.write_magnitude("mag.csv");
    println!("obj:{}", pso.group_best.evaluation);
}
*/
