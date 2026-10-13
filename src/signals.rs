use bc_utils::other::{transpose_set, vec_len_sync_set};
use bc_utils_lg::prelude::*;

use crate::{impl_deref, impl_gw_src_single, impl_gw_src_vec};

#[derive(Debug, Default, Clone)]
pub struct SignalsGw(pub Vec<Vec<Signal>>);

impl SignalsGw {
    pub fn push(&mut self, signals: &MAP<&str, Vec<Signal>>, used: &[String]) {
        for used_signal in used {
            self.0.push(signals[used_signal.as_str()].clone());
        }
    }
    pub fn len_sync(&mut self) {
        vec_len_sync_set(&mut self.0);
    }

    pub fn transpose(&mut self) {
        transpose_set(&mut self.0)
    }

    pub fn all_check(&mut self) {
        if !self.0.is_empty() {
            self.len_sync();
            self.transpose();
        }
    }
}

impl_gw_src_vec!(SignalsGw, Signal);
impl_deref!(SignalsGw, Vec<Vec<Signal>>);

#[derive(Debug, Default, Clone)]
pub struct SignalsGwSeries(pub Vec<Signal>);

impl SignalsGwSeries {
    pub fn push(&mut self, signals: &MAP<&str, Signal>, used: &[String]) {
        for used_signal in used {
            self.0.push(signals[used_signal.as_str()]);
        }
    }
}

impl_gw_src_single!(SignalsGwSeries, Signal);
impl_deref!(SignalsGwSeries, Vec<Signal>);
