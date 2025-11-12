use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::model::Var;

#[derive(Debug, Clone, Default)]
pub struct LinExpr {
    vars: Vec<Var>,
    coeffs: Vec<f64>,
    offset: f64,
}

impl<'a> From<&'a Var> for LinExpr {
    fn from(var: &Var) -> LinExpr {
        LinExpr::new() + var
    }
}

impl From<Var> for LinExpr {
    fn from(var: Var) -> LinExpr {
        LinExpr::from(&var)
    }
}

impl From<f64> for LinExpr {
    fn from(offset: f64) -> LinExpr {
        LinExpr::new() + offset
    }
}

impl Into<(Vec<i32>, Vec<f64>, f64)> for LinExpr {
    fn into(self) -> (Vec<i32>, Vec<f64>, f64) {
        (self.vars.iter().map(|v| v.0).collect(), self.coeffs, self.offset)
    }
}

impl LinExpr {
    /// Create an empty linear expression.
    pub fn new() -> Self {
        LinExpr::default()
    }

    /// Add a linear term into the expression.
    pub fn add_term(mut self, coeff: f64, var: Var) -> Self {
        self.coeffs.push(coeff);
        self.vars.push(var);
        self
    }

    /// Add linear terms into the expression. Panics if the lengths do not match.
    pub fn add_terms(mut self, coeffs: &[f64], vars: &[Var]) -> Self {
        assert_eq!(coeffs.len(), vars.len());
        self.coeffs.extend_from_slice(coeffs);
        self.vars.extend_from_slice(vars);
        self
    }

    /// Add a constant into the expression.
    pub fn add_constant(mut self, constant: f64) -> Self {
        self.offset += constant;
        self
    }

    // /// Get actual value of the expression.
    // pub fn get_value(&self, model: &Model) -> crate::error::Result<f64> {
    //   let vars = model.get_values(attr::X, self.vars.as_slice())?;

    //   Ok(Zip::new((vars, self.coeff.iter())).fold(0.0, |acc, (ind, val)| acc + ind * val) + self.offset)
    // }
}

//   /// `Var` + `Var`  => `LinExpr`
impl Add for Var {
    type Output = LinExpr;
    fn add(self, rhs: Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self).add_term(1.0, rhs)
    }
}
impl<'a> Add<&'a Var> for Var {
    type Output = LinExpr;
    fn add(self, rhs: &Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self).add_term(1.0, rhs.clone())
    }
}
impl<'a> Add<Var> for &'a Var {
    type Output = LinExpr;
    fn add(self, rhs: Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self.clone()).add_term(1.0, rhs)
    }
}
impl<'a, 'b> Add<&'b Var> for &'a Var {
    type Output = LinExpr;
    fn add(self, rhs: &Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self.clone()).add_term(1.0, rhs.clone())
    }
}
impl Add<f64> for Var {
    type Output = LinExpr;
    fn add(self, rhs: f64) -> LinExpr {
        LinExpr::new() + self + rhs
    }
}
impl<'a> Add<f64> for &'a Var {
    type Output = LinExpr;
    fn add(self, rhs: f64) -> LinExpr {
        LinExpr::new() + self.clone() + rhs
    }
}

/// `Var` - `Var` => `LinExpr`
impl Sub for Var {
    type Output = LinExpr;
    fn sub(self, rhs: Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self).add_term(-1.0, rhs)
    }
}
impl<'a> Sub<&'a Var> for Var {
    type Output = LinExpr;
    fn sub(self, rhs: &Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self).add_term(-1.0, rhs.clone())
    }
}
impl<'a> Sub<Var> for &'a Var {
    type Output = LinExpr;
    fn sub(self, rhs: Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self.clone()).add_term(-1.0, rhs)
    }
}
impl<'a, 'b> Sub<&'b Var> for &'a Var {
    type Output = LinExpr;
    fn sub(self, rhs: &Var) -> LinExpr {
        LinExpr::new().add_term(1.0, self.clone()).add_term(-1.0, rhs.clone())
    }
}
impl Sub<LinExpr> for Var {
    type Output = LinExpr;
    fn sub(self, expr: LinExpr) -> LinExpr {
        self + (-expr)
    }
}
impl<'a> Sub<LinExpr> for &'a Var {
    type Output = LinExpr;
    fn sub(self, expr: LinExpr) -> LinExpr {
        self.clone() + (-expr)
    }
}
impl Sub<Var> for f64 {
    type Output = LinExpr;
    fn sub(self, rhs: Var) -> LinExpr {
        LinExpr::new() + self + (-rhs)
    }
}
impl<'a> Sub<&'a Var> for f64 {
    type Output = LinExpr;
    fn sub(self, rhs: &Var) -> LinExpr {
        LinExpr::new() + self + (-rhs.clone())
    }
}

/// -`Var` => `LinExpr`
impl Neg for Var {
    type Output = LinExpr;
    fn neg(self) -> LinExpr {
        LinExpr::new().add_term(-1.0, self)
    }
}
impl<'a> Neg for &'a Var {
    type Output = LinExpr;
    fn neg(self) -> LinExpr {
        LinExpr::new().add_term(-1.0, self.clone())
    }
}

/// `Var` * `f64` => `LinExpr`
impl Mul<f64> for Var {
    type Output = LinExpr;
    fn mul(self, rhs: f64) -> Self::Output {
        LinExpr::new().add_term(rhs, self)
    }
}
impl<'a> Mul<f64> for &'a Var {
    type Output = LinExpr;
    fn mul(self, rhs: f64) -> Self::Output {
        LinExpr::new().add_term(rhs, self.clone())
    }
}
impl Mul<Var> for f64 {
    type Output = LinExpr;
    fn mul(self, rhs: Var) -> Self::Output {
        LinExpr::new().add_term(self, rhs)
    }
}
impl<'a> Mul<&'a Var> for f64 {
    type Output = LinExpr;
    fn mul(self, rhs: &'a Var) -> Self::Output {
        LinExpr::new().add_term(self, rhs.clone())
    }
}

/// `LinExpr` + `Var` => `LinExpr`
impl Add<LinExpr> for Var {
    type Output = LinExpr;
    fn add(self, rhs: LinExpr) -> LinExpr {
        rhs.add_term(1.0, self)
    }
}
impl<'a> Add<LinExpr> for &'a Var {
    type Output = LinExpr;
    fn add(self, rhs: LinExpr) -> LinExpr {
        rhs.add_term(1.0, self.clone())
    }
}
impl Add<Var> for LinExpr {
    type Output = LinExpr;
    fn add(self, rhs: Var) -> LinExpr {
        self.add_term(1.0, rhs)
    }
}
impl<'a> Add<&'a Var> for LinExpr {
    type Output = LinExpr;
    fn add(self, rhs: &'a Var) -> LinExpr {
        self.add_term(1.0, rhs.clone())
    }
}

/// `LinExpr` + `f64` => `LinExpr`
impl Add<f64> for LinExpr {
    type Output = LinExpr;
    fn add(self, rhs: f64) -> Self::Output {
        self.add_constant(rhs)
    }
}
impl Add<LinExpr> for f64 {
    type Output = LinExpr;
    fn add(self, rhs: LinExpr) -> Self::Output {
        rhs.add_constant(self)
    }
}

/// `LinExpr` - `f64` => `LinExpr`
impl Sub<f64> for LinExpr {
    type Output = LinExpr;
    fn sub(self, rhs: f64) -> Self::Output {
        self.add_constant(-rhs)
    }
}

/// `f64` - `LinExpr` => `LinExpr`
impl Sub<LinExpr> for f64 {
    type Output = LinExpr;
    fn sub(self, rhs: LinExpr) -> Self::Output {
        self + (-rhs)
    }
}

impl Add for LinExpr {
    type Output = LinExpr;
    fn add(mut self, rhs: LinExpr) -> Self::Output {
        self += rhs;
        self
    }
}

impl Neg for LinExpr {
    type Output = LinExpr;
    fn neg(mut self) -> LinExpr {
        for coeff in &mut self.coeffs {
            *coeff = -*coeff;
        }
        self.offset = -self.offset;
        self
    }
}

impl AddAssign for LinExpr {
    fn add_assign(&mut self, rhs: LinExpr) {
        for (var, &coeff) in rhs.vars.into_iter().zip(rhs.coeffs.iter()) {
            if let Some(idx) = self.vars.iter().position(|v| *v == var) {
                self.coeffs[idx] += coeff;
            } else {
                self.vars.push(var);
                self.coeffs.push(coeff);
            }
        }
        self.offset += rhs.offset;
    }
}

impl AddAssign<Var> for LinExpr {
    fn add_assign(&mut self, rhs: Var) {
        let expr: LinExpr = rhs.into();
        *self += expr;
    }
}

impl Sub for LinExpr {
    type Output = LinExpr;
    fn sub(self, rhs: LinExpr) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for LinExpr {
    type Output = LinExpr;
    fn mul(mut self, rhs: f64) -> Self::Output {
        for coeff in &mut self.coeffs {
            *coeff *= rhs;
        }
        self.offset *= rhs;
        self
    }
}

impl Div<f64> for LinExpr {
    type Output = LinExpr;
    fn div(mut self, rhs: f64) -> Self::Output {
        for coeff in &mut self.coeffs {
            *coeff /= rhs;
        }
        self.offset /= rhs;
        self
    }
}

impl Mul<LinExpr> for f64 {
    type Output = LinExpr;
    fn mul(self, rhs: LinExpr) -> Self::Output {
        rhs * self
    }
}

impl Sum for LinExpr {
    fn sum<I: Iterator<Item = LinExpr>>(iter: I) -> LinExpr {
        iter.fold(LinExpr::new(), |acc, expr| acc + expr)
    }
}
