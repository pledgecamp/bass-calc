//! A widget for plotting a BassCalc graph
//! Based on https://github.com/PistonDevelopers/conrod/blob/master/src/widget/plot_path.rs

use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget};
use num;
use conrod::{widget, utils};
use parameters::Parameters;
use graphics::graph_fns::bass_fn_point;

/// A widget that plots a BassCalc function, which depends on `Parameters`
///
/// The function returns a list of points, which the widget uses to draw lines according
/// to the current X and Y scales
///
/// The resulting "path" is drawn using conrod's `PointPath` primitive widget.
#[derive(WidgetCommon)]
pub struct BassGraph<F> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
    min_freq: f64,
    max_freq: f64,
    step: f64,
    params: Parameters,
    f: F,
}

/// Unique styling parameters for the `BassGraph` widget.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// The thickness of the plotted line.
    #[conrod(default = "1.0")]
    pub thickness: Option<Scalar>,
    /// The color of the line.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
}

widget_ids! {
    struct Ids {
        point_path,
    }
}

/// Unique state stored between updates for the `BassGraph` widget.
pub struct State {
    ids: Ids,
}


impl<F> BassGraph<F> {
    /// Begin building a new `BassGraph` widget instance.
    pub fn new(min_freq: f64, max_freq: f64, step: f64, params: Parameters, f: F) -> Self {
        BassGraph {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            min_freq: min_freq,
            max_freq: max_freq,
            step: step,
            params: params,
            f: f,
        }
    }

    /// The thickness of the point path used to draw the plot.
    pub fn thickness(mut self, thickness: Scalar) -> Self {
        self.style.thickness = Some(thickness);
        self
    }
}


impl<F> Widget for BassGraph<F>
          where F: Fn(params: &Parameters, x_min) -> Vec<f64>,
{
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the BassGraph.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {

        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let BassGraph { min_freq, max_freq, step, f, params, .. } = self;

        let y_to_scalar =
            |y| utils::map_range(y, -1.0, 1.0, rect.bottom(), rect.top());
        let scalar_to_x =
            |s| utils::map_range(s, rect.left(), rect.right(), min_freq.clone(), max_freq.clone());

        let data = f(&params);
        let point_iter = (0 .. rect.w() as usize)
            .map(|x_scalar| {
                let x_scalar = x_scalar as Scalar + rect.x.start;
                let x = scalar_to_x(x_scalar);
                let y = bass_fn_point(&data, x);
                let y_scalar = y_to_scalar(y);
                [x_scalar, y_scalar]
            });

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());
        widget::PointPath::new(point_iter)
            .wh(rect.dim())
            .xy(rect.xy())
            .color(color)
            .thickness(thickness)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.point_path, ui);
    }

}

impl<F> Colorable for BassGraph<F> {
    builder_method!(color { style.color = Some(Color) });
}
