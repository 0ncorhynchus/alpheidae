#[doc(hidden)]
#[macro_export]
macro_rules! _opt_param {
    ($self:ident, $request:ident, $var:ident) => {
        if let Some(param) = $self.$var {
            $request.parameter(stringify!($var), param);
        }
    };
    ($self:ident, $request:ident, $var:ident[]) => {
        if !$self.$var.is_empty() {
            $request.parameter(
                stringify!($var),
                $self
                    .$var
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _opt_query {
    ($self:ident, $request:ident, $var:ident) => {
        if let Some(param) = $self.$var {
            $request.query(stringify!($var), param);
        }
    };
    ($self:ident, $request:ident, $var:ident[]) => {
        if !$self.$var.is_empty() {
            $request.query(
                stringify!($var),
                $self
                    .$var
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
    };
}
