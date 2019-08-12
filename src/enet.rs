#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register"]
    pub mac_config: MAC_CONFIG,
    #[doc = "0x04 - no description available"]
    pub mac_ext_config: MAC_EXT_CONFIG,
    #[doc = "0x08 - MAC frame filter register"]
    pub mac_frame_filter: MAC_FRAME_FILTER,
    #[doc = "0x0c - MAC watchdog Timeout register"]
    pub mac_wd_timerout: MAC_WD_TIMEROUT,
    _reserved4: [u8; 64usize],
    #[doc = "0x50 - MAC vlan tag register"]
    pub mac_vlan_tag: MAC_VLAN_TAG,
    _reserved5: [u8; 28usize],
    #[doc = "0x70 - Transmit flow control register"]
    pub mac_tx_flow_ctrl_q: [MAC_TX_FLOW_CTRL_Q; 2],
    _reserved6: [u8; 24usize],
    #[doc = "0x90 - Receive flow control register"]
    pub mac_rx_flow_ctrl: MAC_RX_FLOW_CTRL,
    _reserved7: [u8; 4usize],
    #[doc = "0x98 - no description available"]
    pub mac_txq_prio_map: MAC_TXQ_PRIO_MAP,
    _reserved8: [u8; 4usize],
    #[doc = "0xa0 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl0: MAC_RXQ_CTRL0,
    #[doc = "0xa4 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl1: MAC_RXQ_CTRL1,
    #[doc = "0xa8 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl2: MAC_RXQ_CTRL2,
    _reserved11: [u8; 4usize],
    #[doc = "0xb0 - Interrupt status register 0x0000"]
    pub mac_intr_stat: MAC_INTR_STAT,
    #[doc = "0xb4 - Interrupt enable register 0x0000"]
    pub mac_intr_en: MAC_INTR_EN,
    #[doc = "0xb8 - Receive Transmit Status register"]
    pub mac_rxtx_stat: MAC_RXTX_STAT,
    _reserved14: [u8; 4usize],
    #[doc = "0xc0 - no description available"]
    pub mac_pmt_crtl_stat: MAC_PMT_CRTL_STAT,
    #[doc = "0xc4 - Remote wake-up frame filter"]
    pub mac_rwake_frflt: MAC_RWAKE_FRFLT,
    _reserved16: [u8; 8usize],
    #[doc = "0xd0 - LPI Control and Status Register"]
    pub mac_lpi_ctrl_stat: MAC_LPI_CTRL_STAT,
    #[doc = "0xd4 - LPI Timers Control register"]
    pub mac_lpi_timer_ctrl: MAC_LPI_TIMER_CTRL,
    #[doc = "0xd8 - LPI entry Timer register"]
    pub mac_lpi_entr_timr: MAC_LPI_ENTR_TIMR,
    #[doc = "0xdc - no description available"]
    pub mac_1us_tic_countr: MAC_1US_TIC_COUNTR,
    _reserved20: [u8; 48usize],
    #[doc = "0x110 - MAC version register"]
    pub mac_version: MAC_VERSION,
    #[doc = "0x114 - MAC debug register"]
    pub mac_dbg: MAC_DBG,
    _reserved22: [u8; 4usize],
    #[doc = "0x11c - MAC hardware feature register 0x0201"]
    pub mac_hw_feat0: MAC_HW_FEAT0,
    #[doc = "0x120 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat1: MAC_HW_FEAT1,
    #[doc = "0x124 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat2: MAC_HW_FEAT2,
    _reserved25: [u8; 216usize],
    #[doc = "0x200 - MIDO address Register"]
    pub mac_mdio_addr: MAC_MDIO_ADDR,
    #[doc = "0x204 - MDIO Data register"]
    pub mac_mdio_data: MAC_MDIO_DATA,
    _reserved27: [u8; 248usize],
    #[doc = "0x300 - MAC address0 high register"]
    pub mac_addr_high: MAC_ADDR_HIGH,
    #[doc = "0x304 - MAC address0 low register"]
    pub mac_addr_low: MAC_ADDR_LOW,
    _reserved29: [u8; 2040usize],
    #[doc = "0xb00 - Time stamp control register"]
    pub mac_timestamp_ctrl: MAC_TIMESTAMP_CTRL,
    #[doc = "0xb04 - Sub-second increment register"]
    pub mac_sub_scnd_incr: MAC_SUB_SCND_INCR,
    #[doc = "0xb08 - System time seconds register"]
    pub mac_sys_time_scnd: MAC_SYS_TIME_SCND,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub mac_sys_time_nscnd: MAC_SYS_TIME_NSCND,
    #[doc = "0xb10 - no description available"]
    pub mac_sys_time_scnd_upd: MAC_SYS_TIME_SCND_UPD,
    #[doc = "0xb14 - no description available"]
    pub mac_sys_time_nscnd_upd: MAC_SYS_TIME_NSCND_UPD,
    #[doc = "0xb18 - Time stamp addend register"]
    pub mac_sys_timestmp_addend: MAC_SYS_TIMESTMP_ADDEND,
    #[doc = "0xb1c - no description available"]
    pub mac_sys_time_hword_scnd: MAC_SYS_TIME_HWORD_SCND,
    #[doc = "0xb20 - Time stamp status register"]
    pub mac_sys_timestmp_stat: MAC_SYS_TIMESTMP_STAT,
    _reserved38: [u8; 12usize],
    #[doc = "0xb30 - Tx timestamp status nanoseconds"]
    pub mac_tx_timestamp_status_nanoseconds: MAC_TX_TIMESTAMP_STATUS_NANOSECONDS,
    #[doc = "0xb34 - Tx timestamp status seconds"]
    pub mac_tx_timestamp_status_seconds: MAC_TX_TIMESTAMP_STATUS_SECONDS,
    _reserved40: [u8; 32usize],
    #[doc = "0xb58 - Timestamp ingress correction"]
    pub mac_timestamp_ingress_corr_nanosecond: MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND,
    #[doc = "0xb5c - Timestamp egress correction"]
    pub mac_timestamp_egress_corr_nanosecond: MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND,
    _reserved42: [u8; 160usize],
    #[doc = "0xc00 - MTL Operation Mode Register"]
    pub mtl_op_mode: MTL_OP_MODE,
    _reserved43: [u8; 28usize],
    #[doc = "0xc20 - MTL Interrupt Status register"]
    pub mtl_intr_stat: MTL_INTR_STAT,
    _reserved44: [u8; 12usize],
    #[doc = "0xc30 - MTL Receive Queue and DMA Channel Mapping register"]
    pub mtl_rxq_dma_map: MTL_RXQ_DMA_MAP,
    _reserved45: [u8; 204usize],
    #[doc = "0xd00 - no description available"]
    pub mtl_queue: [MTL_QUEUE; 2],
    _reserved46: [u8; 640usize],
    #[doc = "0x1000 - DMA mode register"]
    pub dma_mode: DMA_MODE,
    #[doc = "0x1004 - DMA System Bus mode"]
    pub dma_sysbus_mode: DMA_SYSBUS_MODE,
    #[doc = "0x1008 - DMA Interrupt status"]
    pub dma_intr_stat: DMA_INTR_STAT,
    #[doc = "0x100c - DMA Debug Status"]
    pub dma_dbg_stat: DMA_DBG_STAT,
    _reserved50: [u8; 240usize],
    #[doc = "0x1100 - no description available"]
    pub dma_ch0: DMA_CH,
    _reserved51: [u8; 28usize],
    #[doc = "0x1180 - no description available"]
    pub dma_ch1: DMA_CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MTL_QUEUE {
    #[doc = "0x00 - MTL TxQx Operation Mode register"]
    pub mtl_txqx_op_mode: self::mtl_queue::MTL_TXQX_OP_MODE,
    #[doc = "0x04 - MTL TxQx Underflow register"]
    pub mtl_txqx_undrflw: self::mtl_queue::MTL_TXQX_UNDRFLW,
    #[doc = "0x08 - MTL TxQx Debug register"]
    pub mtl_txqx_dbg: self::mtl_queue::MTL_TXQX_DBG,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - MTL TxQx ETS control register, only TxQ1 support"]
    pub mtl_txqx_ets_ctrl: self::mtl_queue::MTL_TXQX_ETS_CTRL,
    #[doc = "0x14 - MTL TxQx ETS Status register"]
    pub mtl_txqx_ets_stat: self::mtl_queue::MTL_TXQX_ETS_STAT,
    #[doc = "0x18 - no description available"]
    pub mtl_txqx_qntm_wght: self::mtl_queue::MTL_TXQX_QNTM_WGHT,
    #[doc = "0x1c - MTL TxQx SendSlopCredit register, only TxQ1 support"]
    pub mtl_txqx_sndslp_crdt: self::mtl_queue::MTL_TXQX_SNDSLP_CRDT,
    #[doc = "0x20 - MTL TxQx hiCredit register, only TxQ1 support"]
    pub mtl_txqx_hi_crdt: self::mtl_queue::MTL_TXQX_HI_CRDT,
    #[doc = "0x24 - MTL TxQx loCredit register, only TxQ1 support"]
    pub mtl_txqx_lo_crdt: self::mtl_queue::MTL_TXQX_LO_CRDT,
    _reserved9: [u8; 4usize],
    #[doc = "0x2c - no description available"]
    pub mtl_txqx_intctrl_stat: self::mtl_queue::MTL_TXQX_INTCTRL_STAT,
    #[doc = "0x30 - MTL RxQx Operation Mode register"]
    pub mtl_rxqx_op_mode: self::mtl_queue::MTL_RXQX_OP_MODE,
    #[doc = "0x34 - MTL RxQx Missed Packet Overflow Counter register"]
    pub mtl_rxqx_misspkt_ovrflw_cnt: self::mtl_queue::MTL_RXQX_MISSPKT_OVRFLW_CNT,
    #[doc = "0x38 - MTL RxQx Debug register"]
    pub mtl_rxqx_dbg: self::mtl_queue::MTL_RXQX_DBG,
    #[doc = "0x3c - MTL RxQx Control register"]
    pub mtl_rxqx_ctrl: self::mtl_queue::MTL_RXQX_CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod mtl_queue;
#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_CH {
    #[doc = "0x00 - DMA Channelx Control"]
    pub dma_chx_ctrl: self::dma_ch::DMA_CHX_CTRL,
    #[doc = "0x04 - DMA Channelx Transmit Control"]
    pub dma_chx_tx_ctrl: self::dma_ch::DMA_CHX_TX_CTRL,
    #[doc = "0x08 - DMA Channelx Receive Control"]
    pub dma_chx_rx_ctrl: self::dma_ch::DMA_CHX_RX_CTRL,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - no description available"]
    pub dma_chx_txdesc_list_addr: self::dma_ch::DMA_CHX_TXDESC_LIST_ADDR,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - no description available"]
    pub dma_chx_rxdesc_list_addr: self::dma_ch::DMA_CHX_RXDESC_LIST_ADDR,
    #[doc = "0x20 - no description available"]
    pub dma_chx_txdesc_tail_ptr: self::dma_ch::DMA_CHX_TXDESC_TAIL_PTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x28 - no description available"]
    pub dma_chx_rxdesc_tail_ptr: self::dma_ch::DMA_CHX_RXDESC_TAIL_PTR,
    #[doc = "0x2c - no description available"]
    pub dma_chx_txdesc_ring_length: self::dma_ch::DMA_CHX_TXDESC_RING_LENGTH,
    #[doc = "0x30 - Channelx Rx descriptor Ring Length"]
    pub dma_chx_rxdesc_ring_length: self::dma_ch::DMA_CHX_RXDESC_RING_LENGTH,
    #[doc = "0x34 - Channelx Interrupt Enable"]
    pub dma_chx_int_en: self::dma_ch::DMA_CHX_INT_EN,
    #[doc = "0x38 - Receive Interrupt Watchdog Timer"]
    pub dma_chx_rx_int_wdtimer: self::dma_ch::DMA_CHX_RX_INT_WDTIMER,
    #[doc = "0x3c - Slot Function Control and Status"]
    pub dma_chx_slot_func_ctrl_stat: self::dma_ch::DMA_CHX_SLOT_FUNC_CTRL_STAT,
    _reserved12: [u8; 4usize],
    #[doc = "0x44 - Channelx Current Host Transmit descriptor"]
    pub dma_chx_cur_hst_txdesc: self::dma_ch::DMA_CHX_CUR_HST_TXDESC,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - no description available"]
    pub dma_chx_cur_hst_rxdesc: self::dma_ch::DMA_CHX_CUR_HST_RXDESC,
    _reserved14: [u8; 4usize],
    #[doc = "0x54 - no description available"]
    pub dma_chx_cur_hst_txbuf: self::dma_ch::DMA_CHX_CUR_HST_TXBUF,
    _reserved15: [u8; 4usize],
    #[doc = "0x5c - Channelx Current Application Receive Buffer Address"]
    pub dma_chx_cur_hst_rxbuf: self::dma_ch::DMA_CHX_CUR_HST_RXBUF,
    #[doc = "0x60 - Channelx DMA status register"]
    pub dma_chx_stat: self::dma_ch::DMA_CHX_STAT,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod dma_ch;
#[doc = "MAC configuration register"]
pub struct MAC_CONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC configuration register"]
pub mod mac_config;
#[doc = "no description available"]
pub struct MAC_EXT_CONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_ext_config;
#[doc = "MAC frame filter register"]
pub struct MAC_FRAME_FILTER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC frame filter register"]
pub mod mac_frame_filter;
#[doc = "MAC watchdog Timeout register"]
pub struct MAC_WD_TIMEROUT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC watchdog Timeout register"]
pub mod mac_wd_timerout;
#[doc = "MAC vlan tag register"]
pub struct MAC_VLAN_TAG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC vlan tag register"]
pub mod mac_vlan_tag;
#[doc = "Transmit flow control register"]
pub struct MAC_TX_FLOW_CTRL_Q {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit flow control register"]
pub mod mac_tx_flow_ctrl_q;
#[doc = "Receive flow control register"]
pub struct MAC_RX_FLOW_CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive flow control register"]
pub mod mac_rx_flow_ctrl;
#[doc = "no description available"]
pub struct MAC_TXQ_PRIO_MAP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_txq_prio_map;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub struct MAC_RXQ_CTRL0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl0;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub struct MAC_RXQ_CTRL1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl1;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub struct MAC_RXQ_CTRL2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl2;
#[doc = "Interrupt status register 0x0000"]
pub struct MAC_INTR_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register 0x0000"]
pub mod mac_intr_stat;
#[doc = "Interrupt enable register 0x0000"]
pub struct MAC_INTR_EN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register 0x0000"]
pub mod mac_intr_en;
#[doc = "Receive Transmit Status register"]
pub struct MAC_RXTX_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Transmit Status register"]
pub mod mac_rxtx_stat;
#[doc = "no description available"]
pub struct MAC_PMT_CRTL_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_pmt_crtl_stat;
#[doc = "Remote wake-up frame filter"]
pub struct MAC_RWAKE_FRFLT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Remote wake-up frame filter"]
pub mod mac_rwake_frflt;
#[doc = "LPI Control and Status Register"]
pub struct MAC_LPI_CTRL_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LPI Control and Status Register"]
pub mod mac_lpi_ctrl_stat;
#[doc = "LPI Timers Control register"]
pub struct MAC_LPI_TIMER_CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LPI Timers Control register"]
pub mod mac_lpi_timer_ctrl;
#[doc = "LPI entry Timer register"]
pub struct MAC_LPI_ENTR_TIMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LPI entry Timer register"]
pub mod mac_lpi_entr_timr;
#[doc = "no description available"]
pub struct MAC_1US_TIC_COUNTR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_1us_tic_countr;
#[doc = "MAC version register"]
pub struct MAC_VERSION {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC version register"]
pub mod mac_version;
#[doc = "MAC debug register"]
pub struct MAC_DBG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC debug register"]
pub mod mac_dbg;
#[doc = "MAC hardware feature register 0x0201"]
pub struct MAC_HW_FEAT0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat0;
#[doc = "MAC hardware feature register 0x0201"]
pub struct MAC_HW_FEAT1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat1;
#[doc = "MAC hardware feature register 0x0201"]
pub struct MAC_HW_FEAT2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat2;
#[doc = "MIDO address Register"]
pub struct MAC_MDIO_ADDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MIDO address Register"]
pub mod mac_mdio_addr;
#[doc = "MDIO Data register"]
pub struct MAC_MDIO_DATA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MDIO Data register"]
pub mod mac_mdio_data;
#[doc = "MAC address0 high register"]
pub struct MAC_ADDR_HIGH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC address0 high register"]
pub mod mac_addr_high;
#[doc = "MAC address0 low register"]
pub struct MAC_ADDR_LOW {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MAC address0 low register"]
pub mod mac_addr_low;
#[doc = "Time stamp control register"]
pub struct MAC_TIMESTAMP_CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time stamp control register"]
pub mod mac_timestamp_ctrl;
#[doc = "Sub-second increment register"]
pub struct MAC_SUB_SCND_INCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Sub-second increment register"]
pub mod mac_sub_scnd_incr;
#[doc = "System time seconds register"]
pub struct MAC_SYS_TIME_SCND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System time seconds register"]
pub mod mac_sys_time_scnd;
#[doc = "System time nanoseconds register"]
pub struct MAC_SYS_TIME_NSCND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System time nanoseconds register"]
pub mod mac_sys_time_nscnd;
#[doc = "no description available"]
pub struct MAC_SYS_TIME_SCND_UPD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_sys_time_scnd_upd;
#[doc = "no description available"]
pub struct MAC_SYS_TIME_NSCND_UPD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_sys_time_nscnd_upd;
#[doc = "Time stamp addend register"]
pub struct MAC_SYS_TIMESTMP_ADDEND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time stamp addend register"]
pub mod mac_sys_timestmp_addend;
#[doc = "no description available"]
pub struct MAC_SYS_TIME_HWORD_SCND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod mac_sys_time_hword_scnd;
#[doc = "Time stamp status register"]
pub struct MAC_SYS_TIMESTMP_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time stamp status register"]
pub mod mac_sys_timestmp_stat;
#[doc = "Tx timestamp status nanoseconds"]
pub struct MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx timestamp status nanoseconds"]
pub mod mac_tx_timestamp_status_nanoseconds;
#[doc = "Tx timestamp status seconds"]
pub struct MAC_TX_TIMESTAMP_STATUS_SECONDS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx timestamp status seconds"]
pub mod mac_tx_timestamp_status_seconds;
#[doc = "Timestamp ingress correction"]
pub struct MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timestamp ingress correction"]
pub mod mac_timestamp_ingress_corr_nanosecond;
#[doc = "Timestamp egress correction"]
pub struct MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timestamp egress correction"]
pub mod mac_timestamp_egress_corr_nanosecond;
#[doc = "MTL Operation Mode Register"]
pub struct MTL_OP_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MTL Operation Mode Register"]
pub mod mtl_op_mode;
#[doc = "MTL Interrupt Status register"]
pub struct MTL_INTR_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MTL Interrupt Status register"]
pub mod mtl_intr_stat;
#[doc = "MTL Receive Queue and DMA Channel Mapping register"]
pub struct MTL_RXQ_DMA_MAP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MTL Receive Queue and DMA Channel Mapping register"]
pub mod mtl_rxq_dma_map;
#[doc = "DMA mode register"]
pub struct DMA_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA mode register"]
pub mod dma_mode;
#[doc = "DMA System Bus mode"]
pub struct DMA_SYSBUS_MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA System Bus mode"]
pub mod dma_sysbus_mode;
#[doc = "DMA Interrupt status"]
pub struct DMA_INTR_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt status"]
pub mod dma_intr_stat;
#[doc = "DMA Debug Status"]
pub struct DMA_DBG_STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA Debug Status"]
pub mod dma_dbg_stat;
