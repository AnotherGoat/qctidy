use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use qctidy_estimator::EstimatorAdapter;
use qctidy_facade::EstimationRequest;

use crate::extractor;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(estimate, module)?)?;
    Ok(())
}

#[pyfunction(signature = (circuit, shots=1024))]
fn estimate(python: Python<'_>, circuit: &Bound<'_, PyAny>, shots: usize) -> PyResult<Py<PyAny>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = EstimationRequest::new(extracted.into(), shots);
    let response = python
        .detach(|| qctidy_facade::estimate(&request, &EstimatorAdapter))
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    estimation_to_dict(python, response.estimation())
}

fn estimation_to_dict(
    python: Python<'_>,
    estimation: qctidy_ports::Estimation,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(python);
    dict.set_item("execution_time", estimation.execution_time)?;
    dict.set_item("costs", provider_costs_to_list(python, estimation.costs)?)?;
    Ok(dict.into())
}

fn provider_costs_to_list(
    python: Python<'_>,
    costs: Vec<qctidy_ports::ProviderCostEstimates>,
) -> PyResult<Py<PyList>> {
    let list = PyList::empty(python);
    for provider_costs in costs {
        let dict = PyDict::new(python);
        dict.set_item("provider", provider_costs.provider)?;
        dict.set_item("status", provider_costs.status)?;
        dict.set_item(
            "estimates",
            costs_to_list(python, provider_costs.estimates)?,
        )?;
        list.append(dict)?;
    }
    Ok(list.into())
}

fn costs_to_list(
    python: Python<'_>,
    costs: Vec<qctidy_ports::EstimatedCost>,
) -> PyResult<Py<PyList>> {
    let list = PyList::empty(python);
    for cost in costs {
        let dict = PyDict::new(python);
        dict.set_item("provider", cost.provider)?;
        dict.set_item("plan_name", cost.plan_name)?;
        dict.set_item("price_label", cost.price_label)?;
        dict.set_item("cost_usd", cost.cost_usd)?;
        list.append(dict)?;
    }
    Ok(list.into())
}
