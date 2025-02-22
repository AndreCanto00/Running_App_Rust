// lib.rs
use std::f64::consts::E;

#[derive(Debug)]
pub struct ValidationError(String);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Validation error: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}

fn validate_input(value: f64) -> Result<f64, ValidationError> {
    if value < 0.0 {
        Err(ValidationError(
            "Input values must be non-negative".to_string(),
        ))
    } else {
        Ok(value)
    }
}

/// Calculates TRIMP (Training Impulse) value
pub fn trimp(
    avg_hr: f64,
    max_hr: f64,
    rest_hr: f64,
    workout_duration: f64,
) -> Result<f64, ValidationError> {
    let avg_hr = validate_input(avg_hr)?;
    let max_hr = validate_input(max_hr)?;
    let rest_hr = validate_input(rest_hr)?;
    let workout_duration = validate_input(workout_duration)?;

    let ratio = (avg_hr - rest_hr) / (max_hr - rest_hr);
    let trimp_value = ratio * 0.64 * E.powf(ratio * 1.92) * workout_duration;

    Ok(trimp_value)
}

/// Calculates TRIMP_LT (Lactate Threshold Training Impulse) value
/// Default lt_duration is 60
pub fn trimp_lt(
    lt_hr: f64,
    max_hr: f64,
    rest_hr: f64,
    lt_duration: Option<f64>,
) -> Result<f64, ValidationError> {
    let lt_hr = validate_input(lt_hr)?;
    let max_hr = validate_input(max_hr)?;
    let rest_hr = validate_input(rest_hr)?;
    let lt_duration = validate_input(lt_duration.unwrap_or(60.0))?;

    let ratio = (lt_hr - rest_hr) / (max_hr - rest_hr);
    let trimp_lt_value = ratio * 0.64 * E.powf(ratio * 1.92) * lt_duration;

    Ok(trimp_lt_value)
}

/// Calculates HRRS (Heart Rate Recovery Score) as percentage
pub fn hrrs(
    avg_hr: f64,
    max_hr: f64,
    rest_hr: f64,
    workout_duration: f64,
    lt_hr: f64,
) -> Result<f64, ValidationError> {
    let trimp_value = trimp(avg_hr, max_hr, rest_hr, workout_duration)?;
    let trimp_lt_value = trimp_lt(lt_hr, max_hr, rest_hr, None)?;

    Ok(trimp_value / trimp_lt_value * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        assert!(validate_input(0.0).is_ok());
        assert!(validate_input(100.0).is_ok());
        assert!(validate_input(-1.0).is_err());
    }

    #[test]
    fn test_trimp() {
        let result = trimp(140.0, 190.0, 60.0, 60.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trimp_lt() {
        let result = trimp_lt(170.0, 190.0, 60.0, Some(60.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_hrrs() {
        let result = hrrs(140.0, 190.0, 60.0, 60.0, 170.0);
        assert!(result.is_ok());
    }
}
