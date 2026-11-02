"""Plot the Criterion benchmark results as a compact 4x2 summary figure."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")

import matplotlib.pyplot as plt
from matplotlib.ticker import (
    FixedFormatter,
    FixedLocator,
    FuncFormatter,
    NullLocator,
)

ROOT = Path(__file__).resolve().parent
CRITERION = ROOT / "criterion"

LINE_GROUPS = [
    ("Already Simplified", "Qubits"),
    ("Alternating Cancellation", "Qubits"),
    ("Cancellation Chain", "Qubits"),
    ("Nested Cancellation Chain", "Qubits"),
    ("CNOT Cascade", "Qubits"),
    ("Deep Narrow Circuit", "Time steps"),
    ("Wide Shallow Circuit", "Qubits"),
]
RANDOM_GROUP = "Random Circuit"

COLOR = "#2B7FFF"
ACCENT = "#B71C1C"
GRID_COLOR = "#DDDDDD"
HEADER_COLOR = "#EDF3FF"


def format_value(value: float) -> str:
    """Format a time in milliseconds with about three significant figures."""
    if value >= 100:
        return f"{value:.0f}"
    if value >= 10:
        return f"{value:.1f}"
    if value >= 1:
        return f"{value:.2f}"
    if value >= 0.1:
        return f"{value:.3f}"
    return f"{value:.4g}"


def parameters(group: str) -> list[Path]:
    """Benchmark parameter directories of a group, in numeric order."""
    directories = [
        path
        for path in (CRITERION / group).iterdir()
        if path.is_dir() and path.name != "report"
    ]
    return sorted(directories, key=lambda path: [int(part) for part in path.name.split("x")])


def mean_milliseconds(group: str, parameter: str) -> float:
    """Mean execution time of one benchmark, in milliseconds."""
    estimates = CRITERION / group / parameter / "new" / "estimates.json"
    data = json.loads(estimates.read_text())
    return data["mean"]["point_estimate"] / 1e6


def style_axis(axis: plt.Axes, title: str, x_label: str) -> None:
    axis.set_title(title, fontsize=10, fontweight="bold", pad=6)
    axis.set_xlabel(x_label, fontsize=9)
    axis.set_ylabel("Mean time (ms)", fontsize=9)
    axis.grid(True, which="major", color=GRID_COLOR, linewidth=0.6)
    axis.set_axisbelow(True)
    axis.yaxis.set_major_formatter(FuncFormatter(lambda value, _: f"{value:g}"))
    axis.xaxis.set_minor_locator(NullLocator())
    axis.yaxis.set_minor_locator(NullLocator())
    axis.tick_params(axis="both", which="major", labelsize=8)
    for spine in ("top", "right"):
        axis.spines[spine].set_visible(False)


def endpoint_label(x_value: int, x_label: str, time: float) -> str:
    """Label a data point with its x value and its mean execution time."""
    unit = "q" if x_label == "Qubits" else " steps"
    return f"{x_value}{unit} · {format_value(time)} ms"


def plot_line(axis: plt.Axes, group: str, x_label: str) -> None:
    paths = parameters(group)
    x_values = [int(path.name) for path in paths]
    y_values = [mean_milliseconds(group, path.name) for path in paths]
    positions = list(range(len(x_values)))

    axis.plot(positions, y_values, marker="o", markersize=4.5, linewidth=1.8, color=COLOR)
    axis.scatter(
        [positions[0], positions[-1]],
        [y_values[0], y_values[-1]],
        s=70,
        color=ACCENT,
        edgecolors="white",
        linewidths=1.0,
        zorder=3,
    )
    style_axis(axis, group, x_label)
    axis.xaxis.set_major_locator(FixedLocator(positions))
    axis.xaxis.set_major_formatter(FixedFormatter([str(value) for value in x_values]))
    axis.margins(x=0.06, y=0.12)

    for index, horizontal_alignment, offset in ((0, "left", (4, 8)), (-1, "right", (-4, 8))):
        axis.annotate(
            endpoint_label(x_values[index], x_label, y_values[index]),
            (positions[index], y_values[index]),
            textcoords="offset points",
            xytext=offset,
            ha=horizontal_alignment,
            fontsize=8,
            fontweight="bold",
            color=ACCENT,
        )


def plot_random_table(axis: plt.Axes) -> None:
    paths = parameters(RANDOM_GROUP)
    rows = []

    for path in paths:
        width, height = (int(part) for part in path.name.split("x"))
        time = mean_milliseconds(RANDOM_GROUP, path.name)
        rows.append([str(width), str(height), f"{format_value(time)} ms"])

    axis.axis("off")
    axis.set_title(RANDOM_GROUP, fontsize=10, fontweight="bold", pad=6)
    table = axis.table(
        cellText=rows,
        colLabels=["Width", "Height", "Mean time"],
        cellLoc="center",
        colLoc="center",
        loc="center",
    )
    table.auto_set_font_size(False)
    table.set_fontsize(9)
    table.scale(1.0, 1.35)

    for (row, _column), cell in table.get_celld().items():
        cell.set_edgecolor("#CCCCCC")
        if row == 0:
            cell.set_facecolor(HEADER_COLOR)
            cell.set_text_props(fontweight="bold")


def main() -> None:
    plt.rcParams.update(
        {
            "font.family": "DejaVu Sans",
            "axes.facecolor": "white",
            "figure.facecolor": "white",
            "axes.edgecolor": "#CCCCCC",
        }
    )

    figure, axes = plt.subplots(2, 4, figsize=(15, 7.2))
    flat_axes = axes.ravel()

    for axis, (group, x_label) in zip(flat_axes, LINE_GROUPS):
        plot_line(axis, group, x_label)

    plot_random_table(flat_axes[7])

    figure.tight_layout()
    output = ROOT / "benchmarks_summary.png"
    figure.savefig(output, dpi=200, bbox_inches="tight")
    plt.close(figure)

    print(f"Wrote {output}")
    for group, x_label in LINE_GROUPS:
        values = [
            f"{parameter.name}={format_value(mean_milliseconds(group, parameter.name))}"
            for parameter in parameters(group)
        ]
        print(f"{group} ({x_label}): " + ", ".join(values))
    print(
        "Random Circuit (Width x Height): "
        + ", ".join(
            f"{parameter.name}={format_value(mean_milliseconds(RANDOM_GROUP, parameter.name))}"
            for parameter in parameters(RANDOM_GROUP)
        )
    )


if __name__ == "__main__":
    main()
