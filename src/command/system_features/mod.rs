//! 4 General Commands
pub mod responses;
pub mod types;
use atat::{atat_derive::ATATCmd, ATATCmd, Error};
use heapless::{consts, String, Vec};
use responses::*;
use types::*;

use super::NoResponse;

/// 19.8 Power saving control (Power SaVing) +UPSV
/// Sets the UART power saving configuration, but it has a global effect on the module power saving configuration:
/// - If the power saving is enabled (+UPSV: 1), the UART interface is cyclically enabled and the module enters
/// idle mode automatically whenever possible
/// - If the power saving is disabled (+UPSV: 0), the UART interface is always enabled and the module does not
/// enter idle mode
/// - If the power saving is controlled by the UART RTS line (+UPSV: 2), the UART interface is enabled and the
/// module does not enter idle mode as long as the UART RTS line state is ON
/// - If the power saving is controlled by the UART DTR line (+UPSV: 3), the UART interface is enabled and the
/// module does not enter idle mode as long as the UART DTR line state is ON
#[derive(Clone, ATATCmd)]
#[at_cmd("+UPSV", NoResponse)]
pub struct SetPowerSavingControl {
    #[at_arg(position = 0)]
    mode: PowerSavingMode,
    #[at_arg(position = 1)]
    timeout: Option<Seconds>,
}
#[derive(Clone, ATATCmd)]
#[at_cmd("+UPSV?", PowerSavingControl)]
pub struct GetPowerSavingControl;