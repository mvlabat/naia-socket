use std::time::Duration;

cfg_if! {
    if #[cfg(all(target_arch = "wasm32", feature = "wasm_bindgen"))] {
        // Wasm //

        use std::cmp::Ordering;
        use js_sys::Date;

        /// Represents a specific moment in time
        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        pub struct Instant {
            inner: f64,
        }

        impl Instant {
            /// Creates an Instant from the moment the method is called
            pub fn now() -> Self {
                Instant {
                    inner: Date::now(),
                }
            }

            /// Returns time elapsed since the Instant
            pub fn elapsed(&self) -> Duration {
                let inner_duration = Date::now() - self.inner;
                let seconds: u64 = (inner_duration as u64) / 1000;
                let nanos: u32 = ((inner_duration as u32) % 1000) * 1000000;
                return Duration::new(seconds, nanos);
            }

            /// Returns time until the Instant occurs
            pub fn until(&self) -> Duration {
                let inner_duration = self.inner - Date::now();
                let seconds: u64 = (inner_duration as u64) / 1000;
                let nanos: u32 = ((inner_duration as u32) % 1000) * 1000000;
                return Duration::new(seconds, nanos);
            }

            /// Adds a given number of milliseconds to the Instant
            pub fn add_millis(&mut self, millis: u32) {
                let millis_f64: f64 = millis.into();
                self.inner += millis_f64;
            }
        }

        impl Eq for Instant {}

        impl Ord for Instant {
            fn cmp(&self, other: &Self) -> Ordering {
                if self.inner == other.inner {
                    return Ordering::Equal;
                }
                else if self.inner < other.inner {
                    return Ordering::Less;
                }
                else {
                    return Ordering::Greater;
                }
            }
        }
    }
    else {
        // Linux //
        /// Represents a specific moment in time
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Instant {
            inner: std::time::Instant,
        }

        impl Instant {
            /// Creates an Instant from the moment the method is called
            pub fn now() -> Self {
                Instant {
                    inner: std::time::Instant::now(),
                }
            }

            /// Returns time elapsed since the Instant
            pub fn elapsed(&self) -> Duration {
                self.inner.elapsed()
            }

            /// Returns time until the Instant occurs
            pub fn until(&self) -> Duration {
                return self.inner.duration_since(std::time::Instant::now());
            }

            /// Adds a given number of milliseconds to the Instant
            pub fn add_millis(&mut self, millis: u32) {
                self.inner += Duration::from_millis(millis.into());
            }

            /// Returns inner Instant implementation
            pub fn get_inner(&self) -> std::time::Instant {
                return self.inner.clone();
            }
        }
    }
}
