#![allow(non_camel_case_types)]

use crate::constants::return_values::*;
use crate::types::{void, BOOL, BYTE, DWORD, HCAM, INT, UINT, WORD};
use std::fmt::Debug;
use std::hash::Hash;

/// [`BYTE`]-wise IPv4 address representation structure.
#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct UEYE_ETH_ADDR_IPV4_by {
    pub by1: BYTE,
    pub by2: BYTE,
    pub by3: BYTE,
    pub by4: BYTE,
}

impl PartialEq for UEYE_ETH_ADDR_IPV4_by {
    fn eq(&self, other: &UEYE_ETH_ADDR_IPV4_by) -> bool {
        self.by1 == other.by1
            && self.by2 == other.by2
            && self.by3 == other.by3
            && self.by4 == other.by4
    }
}

/// IPv4 address.
#[repr(C, packed(1))]
#[derive(Copy, Clone, Eq)]
pub union UEYE_ETH_ADDR_IPV4 {
    /// [`BYTE`]-wise representation.
    pub by: UEYE_ETH_ADDR_IPV4_by,

    /// [`DWORD`] hexadecimal representation.
    pub dwAddr: DWORD,
}

impl Debug for UEYE_ETH_ADDR_IPV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dwAddr_aligned = unsafe { self.dwAddr };
        let by = unsafe { self.by };

        f.debug_struct("UEYE_ETH_ADDR_IPV4")
            .field("by", &by)
            .field("dwAddr", &dwAddr_aligned)
            .finish()
    }
}

impl PartialEq for UEYE_ETH_ADDR_IPV4 {
    fn eq(&self, other: &UEYE_ETH_ADDR_IPV4) -> bool {
        let by = unsafe { self.by };
        let other_by = unsafe { other.by };

        by.eq(&other_by)
    }
}

impl Hash for UEYE_ETH_ADDR_IPV4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let by = unsafe { self.by };
        by.hash(state);
    }
}

/// Ethernet address.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_ADDR_MAC` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html#ueye_eth_addr_mac)
#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UEYE_ETH_ADDR_MAC {
    /// MAC address in hexadecimal format.
    pub abyOctet: [BYTE; 6],
}

/// IP configuration.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_IP_CONFIGURATION` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html#ueye_eth_ip_configuration)
#[repr(C, packed(1))]
#[derive(Copy, Clone, Eq)]
pub struct UEYE_ETH_IP_CONFIGURATION {
    /// IPv4 address in hexadecimal format (_little-endian_).
    ///
    /// If you set `ipAddress` = `0x00000000` (IP address `0.0.0.0`),
    /// the camera is configured for automatic assignment of the IP address.
    pub ipAddress: UEYE_ETH_ADDR_IPV4,

    /// IPv4 subnet mask in hexadecimal format (_little-endian_).
    pub ipSubnetmask: UEYE_ETH_ADDR_IPV4,

    /// (**reserved**)
    reserved: [BYTE; 4],
}

impl Debug for UEYE_ETH_IP_CONFIGURATION {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UEYE_ETH_IP_CONFIGURATION")
            .field("ipAddress", &self.ipAddress)
            .field("ipSubnetmask", &self.ipSubnetmask)
            .finish()
    }
}

impl PartialEq for UEYE_ETH_IP_CONFIGURATION {
    fn eq(&self, other: &UEYE_ETH_IP_CONFIGURATION) -> bool {
        self.ipAddress.eq(&other.ipAddress) && self.ipSubnetmask.eq(&other.ipSubnetmask)
    }
}

/// Status word for current camera status.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_DEVICE_INFO_HEARTBEAT` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_device_info_heartbeat)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum UEYE_ETH_DEVICESTATUS {
    /// Camera is ready to operate.
    IS_ETH_DEVSTATUS_READY_TO_OPERATE = 0x00000001,

    /// Camera is testing current IP address.
    IS_ETH_DEVSTATUS_TESTING_IP_CURRENT = 0x00000002,

    /// Camera is testing persistent IP address.
    IS_ETH_DEVSTATUS_TESTING_IP_PERSISTENT = 0x00000004,

    /// Camera is testing auto config IP range.
    IS_ETH_DEVSTATUS_TESTING_IP_RANGE = 0x00000008,

    /// Current IP address already assigned on the network.
    IS_ETH_DEVSTATUS_INAPPLICABLE_IP_CURRENT = 0x00000010,

    /// Persistent IP address already assigned on the network.
    IS_ETH_DEVSTATUS_INAPPLICABLE_IP_PERSISTENT = 0x00000020,

    /// IP addresses of auto config IP range already assigned on the network.
    IS_ETH_DEVSTATUS_INAPPLICABLE_IP_RANGE = 0x00000040,

    /// Camera has not been initialized (paired).
    IS_ETH_DEVSTATUS_UNPAIRED = 0x00000100,

    /// Camera is being initialized (paired).
    IS_ETH_DEVSTATUS_PAIRING_IN_PROGRESS = 0x00000200,

    /// Camera has been initialized (paired).
    IS_ETH_DEVSTATUS_PAIRED = 0x00000400,

    /// Camera configured for 100 Mbits/s.
    IS_ETH_DEVSTATUS_FORCE_100MBPS = 0x00001000,

    /// Camera supports no uEye COM port.
    IS_ETH_DEVSTATUS_NO_COMPORT = 0x00002000,

    /// Camera is receiving starter firmware.
    IS_ETH_DEVSTATUS_RECEIVING_FW_STARTER = 0x00010000,

    /// Camera is receiving runtime firmware.
    IS_ETH_DEVSTATUS_RECEIVING_FW_RUNTIME = 0x00020000,

    /// Runtime firmware cannot be used.
    IS_ETH_DEVSTATUS_INAPPLICABLE_FW_RUNTIME = 0x00040000,

    /// Starter firmware cannot be used.
    IS_ETH_DEVSTATUS_INAPPLICABLE_FW_STARTER = 0x00080000,

    /// Camera is rebooting runtime firmware.
    IS_ETH_DEVSTATUS_REBOOTING_FW_RUNTIME = 0x00100000,

    /// Camera is rebooting starter firmware.
    IS_ETH_DEVSTATUS_REBOOTING_FW_STARTER = 0x00200000,

    /// Camera is rebooting failsafe firmware.
    IS_ETH_DEVSTATUS_REBOOTING_FW_FAILSAFE = 0x00400000,

    /// Checksum error (error `0`) in runtime firmware.
    IS_ETH_DEVSTATUS_RUNTIME_FW_ERR0 = 0x80000000,
}

/// Heartbeat information transmitted periodically by a device.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_DEVICE_INFO_HEARTBEAT` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_device_info_heartbeat)
#[derive(Copy, Clone, Eq)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_DEVICE_INFO_HEARTBEAT {
    /// Camera's serial number (_string_).
    pub abySerialNumber: [BYTE; 12],

    /// Type of camera series (`0x80` for Gigabit Ethernet uEye).
    pub byDeviceType: BYTE,

    /// User-defined camera ID.
    pub byCameraID: BYTE,

    /// Camera's sensor ID.
    pub wSensorID: WORD,

    /// Image memory size in MB.
    pub wSizeImgMem_MB: WORD,

    /// (**reserved**)
    reserved_1: [BYTE; 2],

    /// Starter firmware version.
    pub dwVerStarterFirmware: DWORD,

    /// Runtime firmware version.
    pub dwVerRuntimeFirmware: DWORD,

    /// Status word for current camera status.
    pub dwStatus: UEYE_ETH_DEVICESTATUS,

    /// (**reserved**)
    reserved_2: [BYTE; 4],

    /// Camera temperature in °Celsius.
    ///
    /// If the value "-127.9 °C" is returned, the camera does not have a temperature sensor.
    ///
    /// # Layout
    /// * **Bit 15**: algebraic sign
    /// * **Bits 14…11**: filled according to algebraic sign
    /// * **Bits 10…4**: temperature (_places before the decimal point_)
    /// * **Bits 3…0**: temperature (_places after the decimal point_)
    pub wTemperature: WORD,

    /// Link bandwidth in Mbits/s.
    pub wLinkSpeed_Mb: WORD,

    /// MAC address of the camera.
    pub macDevice: UEYE_ETH_ADDR_MAC,

    /// COM port offset from 100.
    ///
    /// Valid range: `-99`…`+156`.
    pub wComportOffset: WORD,

    /// Persistent IP configuration.
    pub ipcfgPersistentIpCfg: UEYE_ETH_IP_CONFIGURATION,

    /// Current IP configuration.
    pub ipcfgCurrentIpCfg: UEYE_ETH_IP_CONFIGURATION,

    /// MAC address of the connected PC, if any.
    pub macPairedHost: UEYE_ETH_ADDR_MAC,

    /// (**reserved**)
    reserved_4: [BYTE; 2],

    /// IP address of the connected PC, if any.
    pub ipPairedHostIp: UEYE_ETH_ADDR_IPV4,

    /// First IP address of the auto-configuration range.
    pub ipAutoCfgIpRangeBegin: UEYE_ETH_ADDR_IPV4,

    /// Last IP address of the auto-configuration range.
    pub ipAutoCfgIpRangeEnd: UEYE_ETH_ADDR_IPV4,

    /// The first eight bytes of the user memory.
    pub abyUserSpace: [BYTE; 8],

    /// (**reserved**)
    reserved_5: [BYTE; 84],

    /// (**reserved**)
    reserved_6: [BYTE; 64],
}

impl Debug for UEYE_ETH_DEVICE_INFO_HEARTBEAT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let abySerialNumber_aligned = self.abySerialNumber;
        let byDeviceType_aligned = self.byDeviceType;
        let byCameraID_aligned = self.byCameraID;
        let wSensorID_aligned = self.wSensorID;
        let wSizeImgMem_MB_aligned = self.wSizeImgMem_MB;
        let dwVerStarterFirmware_aligned = self.dwVerStarterFirmware;
        let dwVerRuntimeFirmware_aligned = self.dwVerRuntimeFirmware;
        let dwStatus_aligned = self.dwStatus;
        let wTemperature_aligned = self.wTemperature;
        let wLinkSpeed_Mb_aligned = self.wLinkSpeed_Mb;
        let wComportOffset_aligned = self.wComportOffset;

        f.debug_struct("UEYE_ETH_DEVICE_INFO_HEARTBEAT")
            .field("abySerialNumber", &abySerialNumber_aligned)
            .field("byDeviceType", &byDeviceType_aligned)
            .field("byCameraID", &byCameraID_aligned)
            .field("wSensorID", &wSensorID_aligned)
            .field("wSizeImgMem_MB", &wSizeImgMem_MB_aligned)
            .field("dwVerStarterFirmware", &dwVerStarterFirmware_aligned)
            .field("dwVerRuntimeFirmware", &dwVerRuntimeFirmware_aligned)
            .field("dwStatus", &dwStatus_aligned)
            .field("wTemperature", &wTemperature_aligned)
            .field("wLinkSpeed_Mb", &wLinkSpeed_Mb_aligned)
            .field("macDevice", &self.macDevice)
            .field("wComportOffset", &wComportOffset_aligned)
            .field("ipcfgPersistentIpCfg", &self.ipcfgPersistentIpCfg)
            .field("ipcfgCurrentIpCfg", &self.ipcfgCurrentIpCfg)
            .field("macPairedHost", &self.macPairedHost)
            .field("ipPairedHostIp", &self.ipPairedHostIp)
            .field("ipAutoCfgIpRangeBegin", &self.ipAutoCfgIpRangeBegin)
            .field("ipAutoCfgIpRangeEnd", &self.ipAutoCfgIpRangeEnd)
            .field("abyUserSpace", &self.abyUserSpace)
            .finish()
    }
}

impl PartialEq for UEYE_ETH_DEVICE_INFO_HEARTBEAT {
    fn eq(&self, other: &UEYE_ETH_DEVICE_INFO_HEARTBEAT) -> bool {
        let abySerialNumber_aligned = self.abySerialNumber;
        let byDeviceType_aligned = self.byDeviceType;
        let byCameraID_aligned = self.byCameraID;
        let wSensorID_aligned = self.wSensorID;
        let wSizeImgMem_MB_aligned = self.wSizeImgMem_MB;
        let dwVerStarterFirmware_aligned = self.dwVerStarterFirmware;
        let dwVerRuntimeFirmware_aligned = self.dwVerRuntimeFirmware;
        let dwStatus_aligned = self.dwStatus;
        let wTemperature_aligned = self.wTemperature;
        let wLinkSpeed_Mb_aligned = self.wLinkSpeed_Mb;
        let wComportOffset_aligned = self.wComportOffset;

        let other_abySerialNumber_aligned = other.abySerialNumber;
        let other_byDeviceType_aligned = other.byDeviceType;
        let other_byCameraID_aligned = other.byCameraID;
        let other_wSensorID_aligned = other.wSensorID;
        let other_wSizeImgMem_MB_aligned = other.wSizeImgMem_MB;
        let other_dwVerStarterFirmware_aligned = other.dwVerStarterFirmware;
        let other_dwVerRuntimeFirmware_aligned = other.dwVerRuntimeFirmware;
        let other_dwStatus_aligned = other.dwStatus;
        let other_wTemperature_aligned = other.wTemperature;
        let other_wLinkSpeed_Mb_aligned = other.wLinkSpeed_Mb;
        let other_wComportOffset_aligned = other.wComportOffset;

        abySerialNumber_aligned.eq(&other_abySerialNumber_aligned)
            && byDeviceType_aligned.eq(&other_byDeviceType_aligned)
            && byCameraID_aligned.eq(&other_byCameraID_aligned)
            && wSensorID_aligned.eq(&other_wSensorID_aligned)
            && wSizeImgMem_MB_aligned.eq(&other_wSizeImgMem_MB_aligned)
            && dwVerStarterFirmware_aligned.eq(&other_dwVerStarterFirmware_aligned)
            && dwVerRuntimeFirmware_aligned.eq(&other_dwVerRuntimeFirmware_aligned)
            && dwStatus_aligned.eq(&other_dwStatus_aligned)
            && wTemperature_aligned.eq(&other_wTemperature_aligned)
            && wLinkSpeed_Mb_aligned.eq(&other_wLinkSpeed_Mb_aligned)
            && self.macDevice.eq(&other.macDevice)
            && wComportOffset_aligned.eq(&other_wComportOffset_aligned)
            && self.ipcfgPersistentIpCfg.eq(&other.ipcfgPersistentIpCfg)
            && self.ipcfgCurrentIpCfg.eq(&other.ipcfgCurrentIpCfg)
            && self.macPairedHost.eq(&other.macPairedHost)
            && self.ipPairedHostIp.eq(&other.ipPairedHostIp)
            && self.ipAutoCfgIpRangeBegin.eq(&other.ipAutoCfgIpRangeBegin)
            && self.ipAutoCfgIpRangeEnd.eq(&other.ipAutoCfgIpRangeEnd)
            && self.abyUserSpace.eq(&other.abyUserSpace)
    }
}

/// Status word for driver-based camera management.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_DEVICE_INFO_CONTROL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_device_info_control)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum UEYE_ETH_CONTROLSTATUS {
    /// The camera is available.
    IS_ETH_CTRLSTATUS_AVAILABLE = 0x00000001,

    /// Camera has valid IP address and can be accessed over the network.
    IS_ETH_CTRLSTATUS_ACCESSIBLE1 = 0x00000002,

    /// Camera has no persistent IP address; the auto IP range is valid.
    IS_ETH_CTRLSTATUS_ACCESSIBLE2 = 0x00000004,

    /// Camera can be accessed over the network by its persistent IP address/
    IS_ETH_CTRLSTATUS_PERSISTENT_IP_USED = 0x00000010,

    /// Camera is compatible with the installed driver.
    IS_ETH_CTRLSTATUS_COMPATIBLE = 0x00000020,

    /// DHCP is enabled on the PC network card.
    IS_ETH_CTRLSTATUS_ADAPTER_ON_DHCP = 0x00000040,

    /// The PC network card setup is OK with respect to uEye needs.
    IS_ETH_CTRLSTATUS_ADAPTER_SETUP_OK = 0x00000080,

    /// Camera is being closed on this PC.
    IS_ETH_CTRLSTATUS_UNPAIRING_IN_PROGRESS = 0x00000100,

    /// Camera is being initialized on this PC.
    IS_ETH_CTRLSTATUS_PAIRING_IN_PROGRESS = 0x00000200,

    /// Camera has been initialized on this PC.
    IS_ETH_CTRLSTATUS_PAIRED = 0x00001000,

    /// Camera has been opened on this PC.
    IS_ETH_CTRLSTATUS_OPENED = 0x00004000,

    /// Starter firmware is being loaded onto the camera.
    IS_ETH_CTRLSTATUS_FW_UPLOAD_STARTER = 0x00010000,

    /// Runtime firmware is being loaded onto the camera.
    IS_ETH_CTRLSTATUS_FW_UPLOAD_RUNTIME = 0x00020000,

    /// Camera is rebooting.
    IS_ETH_CTRLSTATUS_REBOOTING = 0x00100000,

    /// Boot-boosting is enabled for this camera.
    IS_ETH_CTRLSTATUS_BOOTBOOST_ENABLED = 0x01000000,

    /// Boot-boosting is active for this camera.
    IS_ETH_CTRLSTATUS_BOOTBOOST_ACTIVE = 0x02000000,

    /// Camera has been initialized in the driver.
    IS_ETH_CTRLSTATUS_INITIALIZED = 0x08000000,

    /// Camera is being removed from driver management.
    IS_ETH_CTRLSTATUS_TO_BE_DELETED = 0x40000000,

    /// Camera is being removed from driver management.
    IS_ETH_CTRLSTATUS_TO_BE_REMOVED = 0x80000000,
}

/// Control information for a listed camera.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_DEVICE_INFO_CONTROL` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_device_info_control)
#[derive(Copy, Clone)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_DEVICE_INFO_CONTROL {
    /// Internal device ID of the camera.
    pub dwDeviceID: DWORD,

    /// Status word for driver-based camera management.
    pub dwControlStatus: UEYE_ETH_CONTROLSTATUS,

    /// (**reserved**)
    reserved_1: [BYTE; 80],

    /// (**reserved**)
    reserved_2: [BYTE; 64],
}

/// [`Debug`] implementation to avoid `reserved` fields.
impl Debug for UEYE_ETH_DEVICE_INFO_CONTROL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dwDeviceID_aligned = self.dwDeviceID;
        let dwControlStatus_aligned = self.dwControlStatus;

        f.debug_struct("UEYE_ETH_DEVICE_INFO_CONTROL")
            .field("dwDeviceID", &dwDeviceID_aligned)
            .field("dwControlStatus", &dwControlStatus_aligned)
            .finish()
    }
}

/// [`PartialEq`] implementation to avoid `reserved` fields.
impl PartialEq for UEYE_ETH_DEVICE_INFO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        let dwDeviceID_aligned = self.dwDeviceID;
        let dwControlStatus_aligned = self.dwControlStatus;

        let other_dwDeviceID_aligned = other.dwDeviceID;
        let other_dwControlStatus_aligned = other.dwControlStatus;

        dwDeviceID_aligned == other_dwDeviceID_aligned
            && dwControlStatus_aligned == other_dwControlStatus_aligned
    }
}

/// Ethernet configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_ETHERNET_CONFIGURATION {
    pub ipcfg: UEYE_ETH_IP_CONFIGURATION,
    pub mac: UEYE_ETH_ADDR_MAC,
}

/// Auto-config IP setup.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_AUTOCFG_IP_SETUP` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html#ueye_eth_autocfg_ip_setup)
#[derive(Debug, Copy, Clone, Eq)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_AUTOCFG_IP_SETUP {
    /// First IPv4 address of the autoconfiguration range in hexadecimal format (_little-endian_).
    pub ipAutoCfgIpRangeBegin: UEYE_ETH_ADDR_IPV4,

    /// Last IPv4 address of the autoconfiguration range in hexadecimal format (_little-endian_).
    pub ipAutoCfgIpRangeEnd: UEYE_ETH_ADDR_IPV4,

    /// (**reserved**)
    reserved: [BYTE; 4],
}

impl PartialEq for UEYE_ETH_AUTOCFG_IP_SETUP {
    fn eq(&self, other: &Self) -> bool {
        self.ipAutoCfgIpRangeBegin == other.ipAutoCfgIpRangeBegin
            && self.ipAutoCfgIpRangeEnd == other.ipAutoCfgIpRangeEnd
    }
}

/// Filter settings for incoming packets.
///
/// <div class="warning">
///
/// ARP and ICMP (ping) packets are always passed!
///
/// </div>
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_ADAPTER_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_adapter_info)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u16)]
pub enum UEYE_ETH_PACKETFILTER_SETUP {
    /// Forward all packets to the operating system.
    IS_ETH_PCKTFLT_PASSALL = 0,

    /// Block GigE uEye data packets directed to the operating system (_recommended_).
    IS_ETH_PCKTFLT_BLOCKUEGET = 1,

    /// Block all packets directed to the operating system.
    IS_ETH_PCKTFLT_BLOCKALL = 2,
}

/// Values for link speed setup.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_ADAPTER_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_adapter_info)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u32)]
pub enum UEYE_ETH_LINKSPEED_SETUP {
    /// 100 Mbits/s.
    IS_ETH_LINKSPEED_100MB = 100,

    /// 1000 Mbits/s.
    IS_ETH_LINKSPEED_1000MB = 1000,
}

/// Control info for a camera's network adapter.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_ADAPTER_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_adapter_info)
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_ADAPTER_INFO {
    /// Network adapter ID as defined internally in the driver.
    pub dwAdapterID: DWORD,

    /// Link speed.
    pub dwDeviceLinkspeed: UEYE_ETH_LINKSPEED_SETUP,

    /// Ethernet configuration of the network adapter.
    pub ethcfg: UEYE_ETH_ETHERNET_CONFIGURATION,

    /// (**reserved**)
    reserved_2: [BYTE; 2],

    /// The adapter is configured for DHCP.
    pub bIsEnabledDHCP: BOOL,

    /// Setting of the IP address autoconfiguration.
    pub autoCfgIp: UEYE_ETH_AUTOCFG_IP_SETUP,

    /// The IP autoconfiguration setting is valid.
    ///
    /// The given range is valid when:
    /// * the beginning and end are valid IP addresses
    /// * the beginning and end are in the subnet of the adapter
    pub bIsValidAutoCfgIpRange: BOOL,

    /// Number of cameras detected at this network adapter.
    pub dwCntDevicesKnown: DWORD,

    /// Number of cameras initialized using this network adapter.
    pub dwCntDevicesPaired: DWORD,

    /// Filter settings for incoming packets.
    pub wPacketFilter: UEYE_ETH_PACKETFILTER_SETUP,

    /// (**reserved**)
    reserved_3: [BYTE; 38],

    /// (**reserved**)
    reserved_4: [BYTE; 64],
}

/// Driver information for the camera's adapter.
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO::UEYE_ETH_DRIVER_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_driver_info)
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_DRIVER_INFO {
    /// Minimum compatible starter firmware version.
    pub dwMinVerStarterFirmware: DWORD,

    /// Maximum compatible starter firmware version.
    pub dwMaxVerStarterFirmware: DWORD,

    /// (**reserved**)
    reserved_1: [BYTE; 8],

    /// (**reserved**)
    reserved_2: [BYTE; 64],
}

/// Ethernet device information structure.
///
/// # Related functions
/// * [`is_GetEthDeviceInfo`]
///
/// # Documentation
/// [Contents of the `UEYE_ETH_DEVICE_INFO` structure](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_deviceinfo.html#ueye_eth_device_info)
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct UEYE_ETH_DEVICE_INFO {
    /// Camera-related data retrieved from the camera (from the heartbeat telegram).
    pub infoDevHeartbeat: UEYE_ETH_DEVICE_INFO_HEARTBEAT,

    /// Camera-related driver data.
    pub infoDevControl: UEYE_ETH_DEVICE_INFO_CONTROL,

    /// Network-card related driver data.
    pub infoAdapter: UEYE_ETH_ADAPTER_INFO,

    /// General driver data.
    pub infoDriver: UEYE_ETH_DRIVER_INFO,
}

unsafe extern "C" {
    /// Set the packet filter for a network adapter.
    ///
    /// <div class="warning">
    /// Only incoming packets are filtered.
    /// Regardless of this setting, ICMP (Ping) and ARP packets are always forwarded
    /// to the operating system.
    /// </div>
    ///
    /// # Input parameters
    /// * `iAdapterID` - Internal adapter ID of the network adapter.
    ///     It is returned by the [`is_DeviceInfo`] function in the
    ///     [`UEYE_ETH_ADAPTER_INFO`] structure.
    /// * `uFilterSetting` - Filter setting for incoming packets.
    ///
    /// # Return values
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [`is_SetPacketFilter`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setpacketfilter.html)
    pub fn is_SetPacketFilter(iAdapterID: INT, uFilterSetting: UEYE_ETH_PACKETFILTER_SETUP) -> INT;
}

/// Enumeration of IP configuration capability flags.
///
/// # Documentation
/// [Contents of the `IPCONFIG_CAPABILITY_FLAGS` enumeration](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html#e_ipconfig_capability_flags)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum IPCONFIG_CAPABILITY_FLAGS {
    /// Setting a persistent IP address is supported.
    IPCONFIG_CAP_PERSISTENT_IP_SUPPORTED = 0x01,

    /// Automatic IP configuration by the network adapter is supported.
    IPCONFIG_CAP_DHCP_SUPPORTED = 0x02,

    /// Obtaining the IP address from a DHCP server is supported.
    IPCONFIG_CAP_AUTOCONFIG_IP_SUPPORTED = 0x04,
}

/// Enumeration of commands supported by the IP configuration access function [`is_IpConfig`].
///
/// # Documentation
/// [`is_IpConfig`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum IPCONFIG_CMD {
    /// Returns the function modes supported by the camera.
    ///
    /// # Parameter type
    /// Pointer to bit mask of type [`UINT`] returning the supported function modes:
    /// * [`IPCONFIG_CAPABILITY_FLAGS::IPCONFIG_CAP_PERSISTENT_IP_SUPPORTED`]
    /// * [`IPCONFIG_CAPABILITY_FLAGS::IPCONFIG_CAP_AUTOCONFIG_IP_SUPPORTED`]
    IPCONFIG_CMD_QUERY_CAPABILITIES = 0,

    /// Sets the persistent IP address and the subnet mask of the camera.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// <div class="warning">
    /// Changing persistent IP configuration enabled status is allowed only if device is not paired.
    /// </div>
    ///
    /// # Parameter type
    /// [`UEYE_ETH_IP_CONFIGURATION`]
    IPCONFIG_CMD_SET_PERSISTENT_IP = 0x01010000,

    /// Activates the DHCP functionality so that a camera obtains the IP address from a DHCP server.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// <div class="warning">
    /// Changing DHCP configuration enabled status is allowed only if device is not paired.
    /// </div>
    ///
    /// # Parameter type
    /// [`IS_U32`], _with bit `0` as boolean active._
    IPCONFIG_CMD_SET_DHCP_ENABLED = 0x01020000,

    /// Sets the IP address range for automatic IP configuration.
    ///
    /// The ID of the network adapter must be passed in `iID` or the MAC address of the
    /// network adapter must be passed in `mac`.
    ///
    /// <div class="warning">
    /// Changed autoconfig IP setup is applied for each device connected to the addressed NIC at
    /// the next pairing of the device.
    /// </div>
    ///
    /// # Parameter type
    /// [`UEYE_ETH_AUTOCFG_IP_SETUP`]
    IPCONFIG_CMD_SET_AUTOCONFIG_IP = 0x01040000,

    /// Sets the IP address range for automatic IP configuration via the device ID.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// <div class="warning">
    /// Changed autoconfig IP setup is applied for each device connected to the addressed NIC at
    /// the next pairing of the device.
    /// </div>
    ///
    /// # Parameter type
    /// [`UEYE_ETH_AUTOCFG_IP_SETUP`]
    IPCONFIG_CMD_SET_AUTOCONFIG_IP_BYDEVICE = 0x01040100,

    /// (**reserved**)
    IPCONFIG_CMD_RESERVED1 = 0x01080000,

    /// Returns the persistent IP address and the subnet mask of the camera.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// # Parameter type
    /// [`UEYE_ETH_IP_CONFIGURATION`]
    IPCONFIG_CMD_GET_PERSISTENT_IP = 0x02010000,

    /// Returns if the DHCP functionality is enabled.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// # Parameter type
    /// [`IS_U32`], _with bit `0` as boolean active._
    IPCONFIG_CMD_GET_DHCP_ENABLED = 0x02020000,

    /// Returns the IP address range for automatic IP configuration.
    ///
    /// The ID of the network adapter must be passed in `iID` or the MAC address of the
    /// network adapter must be passed in `mac`.
    ///
    /// # Parameter type
    /// [`UEYE_ETH_AUTOCFG_IP_SETUP`]
    IPCONFIG_CMD_GET_AUTOCONFIG_IP = 0x02040000,

    /// Returns the IP address range for automatic IP configuration via the device ID.
    ///
    /// The device ID of the camera must be passed in `iID` or the MAC address of the camera must
    /// be passed in `mac`.
    ///
    /// # Parameter type
    /// [`UEYE_ETH_AUTOCFG_IP_SETUP`]
    IPCONFIG_CMD_GET_AUTOCONFIG_IP_BYDEVICE = 0x02040100,
}

unsafe extern "C" {
    /// Allows configuring the IP settings of GigE uEye cameras and the associated
    /// GigE network adapters
    /// (see [IDS Camera Manager](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_kameramanager.html)).
    ///
    /// 1. If a persistent IP address is configured for a GigE camera, it will be used (if free).
    /// 2. If no persistent IP address is configured for a GigE camera, the IP address is obtained
    ///     via DHCP (if DHCP is enabled and a DHCP server is available).
    /// 3. If no persistent IP address is configured for a GigE camera and DHCP is not possible,
    ///     an IP address is obtained from the IP address range for automatic IP configuration.
    ///
    /// <div class="warning">
    ///
    /// The [`is_IpConfig`] function does not accept a camera handle in the `iID` parameter.
    /// In the call, please use the internal device ID as described below.
    ///
    /// </div>
    ///
    /// ## Assigning a persistent IP address to GigE uEye cameras
    /// If you set a persistent IP address, the address is permanently stored in camera memory.
    /// The persistent IP address is retained even after the camera is disconnected from the
    /// power supply. When connecting the camera to a different PC, make sure the persistent
    /// IP address is valid on that PC, as well. If you set the persistent IP address to `0.0.0.0`,
    /// the camera is configured for automatic assignment of the IP address (see below).
    ///
    /// **Note on setting persistent IP addresses:**
    /// Persistent IP addresses can only be set for cameras that have not been initialized
    /// (see also [Pairing](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/hw_begriffe.html?anchor=paired)).
    ///
    /// ## Configuring automatic IP assignment by the network adapter
    /// You can also configure connected GigE uEye cameras to automatically obtain the IP address
    /// from the network adapter (see above). To use this feature, you need to define a suitable
    /// IP address range for the network adapter. When pairing a GigE uEye camera with the
    /// IP address `0.0.0.0`, the uEye driver automatically assigns a free IP address to the camera.
    ///
    /// **Note on using GigE uEye cameras on a DHCP network:**
    /// If a DHCP server is running on the network, you need to ensure when configuring the
    /// network adapter that the manually assigned address range of the uEye driver is outside
    /// the DHCP range.
    ///
    /// ## Obtaining the IP address from a DHCP server
    /// In order to use the DHCP functionality, the camera must be operated with firmware version
    /// 4.95 or higher. For a camera with a firmware version lower than 4.95, the starter firmware
    /// must be updated first, e.g. in IDS Camera Manager via the "Starter Firmware Upload" button.
    ///
    /// # Input parameters
    /// * `iID` - Device ID of the camera.
    ///     You can query the camera's device ID with the [`is_GetCameraList`] function.
    ///     Pass `iID` = `-1` to address the camera or network adapter by its
    ///     MAC address (_recommended_).
    /// * `mac` - If `iID` = `-1`:
    ///     Variable of type [`UEYE_ETH_ADDR_MAC`] containing the MAC address of the camera or
    ///     network adapter in hexadecimal format.
    /// * `nCommand` - Command. See [`IPCONFIG_CMD`].
    /// * `pParam` - Pointer to a function parameter, whose function depends on `nCommand`.
    /// * `cbSizeOfParam` - Size (in bytes) of the memory area to which `pParam` refers.
    ///
    /// # Return values
    /// * [`IS_ACCESS_VIOLATION`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_NOT_SUPPORTED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Documentation
    /// [`is_IpConfig`](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_ipconfig.html)
    pub fn is_IpConfig(
        iID: INT,
        mac: UEYE_ETH_ADDR_MAC,
        nCommand: IPCONFIG_CMD,
        pParam: *mut void,
        cbSizeOfParam: UINT,
    ) -> INT;
}
