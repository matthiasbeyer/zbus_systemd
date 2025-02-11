// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.machine1.Manager`.
#[dbus_proxy(
    interface = "org.freedesktop.machine1.Manager",
    gen_blocking = false,
    default_service = "org.freedesktop.machine1",
    default_path = "/org/freedesktop/machine1"
)]
trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMachine()) Call interface method `GetMachine`.
    #[dbus_proxy(name = "GetMachine")]
    fn get_machine(&self, name: String) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImage()) Call interface method `GetImage`.
    #[dbus_proxy(name = "GetImage")]
    fn get_image(&self, name: String) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMachineByPID()) Call interface method `GetMachineByPID`.
    #[dbus_proxy(name = "GetMachineByPID")]
    fn get_machine_by_pid(&self, pid: u32)
        -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListMachines()) Call interface method `ListMachines`.
    #[dbus_proxy(name = "ListMachines")]
    fn list_machines(
        &self,
    ) -> crate::zbus::Result<Vec<(String, String, String, crate::zvariant::OwnedObjectPath)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListImages()) Call interface method `ListImages`.
    #[dbus_proxy(name = "ListImages")]
    fn list_images(
        &self,
    ) -> crate::zbus::Result<
        Vec<(
            String,
            String,
            bool,
            u64,
            u64,
            u64,
            crate::zvariant::OwnedObjectPath,
        )>,
    >;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CreateMachine()) Call interface method `CreateMachine`.
    #[dbus_proxy(name = "CreateMachine")]
    fn create_machine(
        &self,
        name: String,
        id: Vec<u8>,
        service: String,
        class: String,
        leader: u32,
        root_directory: String,
        scope_properties: Vec<(String, crate::zvariant::OwnedValue)>,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CreateMachineWithNetwork()) Call interface method `CreateMachineWithNetwork`.
    #[dbus_proxy(name = "CreateMachineWithNetwork")]
    fn create_machine_with_network(
        &self,
        name: String,
        id: Vec<u8>,
        service: String,
        class: String,
        leader: u32,
        root_directory: String,
        ifindices: Vec<i32>,
        scope_properties: Vec<(String, crate::zvariant::OwnedValue)>,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RegisterMachine()) Call interface method `RegisterMachine`.
    #[dbus_proxy(name = "RegisterMachine")]
    fn register_machine(
        &self,
        name: String,
        id: Vec<u8>,
        service: String,
        class: String,
        leader: u32,
        root_directory: String,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RegisterMachineWithNetwork()) Call interface method `RegisterMachineWithNetwork`.
    #[dbus_proxy(name = "RegisterMachineWithNetwork")]
    fn register_machine_with_network(
        &self,
        name: String,
        id: Vec<u8>,
        service: String,
        class: String,
        leader: u32,
        root_directory: String,
        ifindices: Vec<i32>,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#UnregisterMachine()) Call interface method `UnregisterMachine`.
    #[dbus_proxy(name = "UnregisterMachine")]
    fn unregister_machine(&self, name: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TerminateMachine()) Call interface method `TerminateMachine`.
    #[dbus_proxy(name = "TerminateMachine")]
    fn terminate_machine(&self, id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#KillMachine()) Call interface method `KillMachine`.
    #[dbus_proxy(name = "KillMachine")]
    fn kill_machine(&self, name: String, who: String, signal: i32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMachineAddresses()) Call interface method `GetMachineAddresses`.
    #[dbus_proxy(name = "GetMachineAddresses")]
    fn get_machine_addresses(&self, name: String) -> crate::zbus::Result<Vec<(i32, Vec<u8>)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMachineOSRelease()) Call interface method `GetMachineOSRelease`.
    #[dbus_proxy(name = "GetMachineOSRelease")]
    fn get_machine_os_release(
        &self,
        name: String,
    ) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenMachinePTY()) Call interface method `OpenMachinePTY`.
    #[dbus_proxy(name = "OpenMachinePTY")]
    fn open_machine_pty(
        &self,
        name: String,
    ) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenMachineLogin()) Call interface method `OpenMachineLogin`.
    #[dbus_proxy(name = "OpenMachineLogin")]
    fn open_machine_login(
        &self,
        name: String,
    ) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenMachineShell()) Call interface method `OpenMachineShell`.
    #[dbus_proxy(name = "OpenMachineShell")]
    fn open_machine_shell(
        &self,
        name: String,
        user: String,
        path: String,
        args: Vec<String>,
        environment: Vec<String>,
    ) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#BindMountMachine()) Call interface method `BindMountMachine`.
    #[dbus_proxy(name = "BindMountMachine")]
    fn bind_mount_machine(
        &self,
        name: String,
        source: String,
        destination: String,
        read_only: bool,
        mkdir: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyFromMachine()) Call interface method `CopyFromMachine`.
    #[dbus_proxy(name = "CopyFromMachine")]
    fn copy_from_machine(
        &self,
        name: String,
        source: String,
        destination: String,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyToMachine()) Call interface method `CopyToMachine`.
    #[dbus_proxy(name = "CopyToMachine")]
    fn copy_to_machine(
        &self,
        name: String,
        source: String,
        destination: String,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyFromMachineWithFlags()) Call interface method `CopyFromMachineWithFlags`.
    #[dbus_proxy(name = "CopyFromMachineWithFlags")]
    fn copy_from_machine_with_flags(
        &self,
        name: String,
        source: String,
        destination: String,
        flags: u64,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyToMachineWithFlags()) Call interface method `CopyToMachineWithFlags`.
    #[dbus_proxy(name = "CopyToMachineWithFlags")]
    fn copy_to_machine_with_flags(
        &self,
        name: String,
        source: String,
        destination: String,
        flags: u64,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenMachineRootDirectory()) Call interface method `OpenMachineRootDirectory`.
    #[dbus_proxy(name = "OpenMachineRootDirectory")]
    fn open_machine_root_directory(
        &self,
        name: String,
    ) -> crate::zbus::Result<crate::zvariant::OwnedFd>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetMachineUIDShift()) Call interface method `GetMachineUIDShift`.
    #[dbus_proxy(name = "GetMachineUIDShift")]
    fn get_machine_uid_shift(&self, name: String) -> crate::zbus::Result<u32>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RemoveImage()) Call interface method `RemoveImage`.
    #[dbus_proxy(name = "RemoveImage")]
    fn remove_image(&self, name: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RenameImage()) Call interface method `RenameImage`.
    #[dbus_proxy(name = "RenameImage")]
    fn rename_image(&self, name: String, new_name: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CloneImage()) Call interface method `CloneImage`.
    #[dbus_proxy(name = "CloneImage")]
    fn clone_image(
        &self,
        name: String,
        new_name: String,
        read_only: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MarkImageReadOnly()) Call interface method `MarkImageReadOnly`.
    #[dbus_proxy(name = "MarkImageReadOnly")]
    fn mark_image_read_only(&self, name: String, read_only: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageHostname()) Call interface method `GetImageHostname`.
    #[dbus_proxy(name = "GetImageHostname")]
    fn get_image_hostname(&self, name: String) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageMachineID()) Call interface method `GetImageMachineID`.
    #[dbus_proxy(name = "GetImageMachineID")]
    fn get_image_machine_id(&self, name: String) -> crate::zbus::Result<Vec<u8>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageMachineInfo()) Call interface method `GetImageMachineInfo`.
    #[dbus_proxy(name = "GetImageMachineInfo")]
    fn get_image_machine_info(
        &self,
        name: String,
    ) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetImageOSRelease()) Call interface method `GetImageOSRelease`.
    #[dbus_proxy(name = "GetImageOSRelease")]
    fn get_image_os_release(
        &self,
        name: String,
    ) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetPoolLimit()) Call interface method `SetPoolLimit`.
    #[dbus_proxy(name = "SetPoolLimit")]
    fn set_pool_limit(&self, size: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetImageLimit()) Call interface method `SetImageLimit`.
    #[dbus_proxy(name = "SetImageLimit")]
    fn set_image_limit(&self, name: String, size: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CleanPool()) Call interface method `CleanPool`.
    #[dbus_proxy(name = "CleanPool")]
    fn clean_pool(&self, mode: String) -> crate::zbus::Result<Vec<(String, u64)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MapFromMachineUser()) Call interface method `MapFromMachineUser`.
    #[dbus_proxy(name = "MapFromMachineUser")]
    fn map_from_machine_user(&self, name: String, uid_inner: u32) -> crate::zbus::Result<u32>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MapToMachineUser()) Call interface method `MapToMachineUser`.
    #[dbus_proxy(name = "MapToMachineUser")]
    fn map_to_machine_user(
        &self,
        uid_outer: u32,
    ) -> crate::zbus::Result<(String, crate::zvariant::OwnedObjectPath, u32)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MapFromMachineGroup()) Call interface method `MapFromMachineGroup`.
    #[dbus_proxy(name = "MapFromMachineGroup")]
    fn map_from_machine_group(&self, name: String, gid_inner: u32) -> crate::zbus::Result<u32>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#MapToMachineGroup()) Call interface method `MapToMachineGroup`.
    #[dbus_proxy(name = "MapToMachineGroup")]
    fn map_to_machine_group(
        &self,
        gid_outer: u32,
    ) -> crate::zbus::Result<(String, crate::zvariant::OwnedObjectPath, u32)>;

    /// Receive `MachineNew` signal.
    #[dbus_proxy(signal, name = "MachineNew")]
    fn machine_new(
        &self,
        machine: String,
        path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `MachineRemoved` signal.
    #[dbus_proxy(signal, name = "MachineRemoved")]
    fn machine_removed(
        &self,
        machine: String,
        path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Get property `PoolPath`.
    #[dbus_proxy(property, name = "PoolPath")]
    fn pool_path(&self) -> crate::zbus::Result<String>;

    /// Get property `PoolUsage`.
    #[dbus_proxy(property, name = "PoolUsage")]
    fn pool_usage(&self) -> crate::zbus::Result<u64>;

    /// Get property `PoolLimit`.
    #[dbus_proxy(property, name = "PoolLimit")]
    fn pool_limit(&self) -> crate::zbus::Result<u64>;
}

/// Proxy object for `org.freedesktop.machine1.Machine`.
#[dbus_proxy(
    interface = "org.freedesktop.machine1.Machine",
    gen_blocking = false,
    default_service = "org.freedesktop.machine1",
    assume_defaults = false
)]
trait Machine {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Terminate()) Call interface method `Terminate`.
    #[dbus_proxy(name = "Terminate")]
    fn terminate(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Kill()) Call interface method `Kill`.
    #[dbus_proxy(name = "Kill")]
    fn kill(&self, who: String, signal: i32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetAddresses()) Call interface method `GetAddresses`.
    #[dbus_proxy(name = "GetAddresses")]
    fn get_addresses(&self) -> crate::zbus::Result<Vec<(i32, Vec<u8>)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetOSRelease()) Call interface method `GetOSRelease`.
    #[dbus_proxy(name = "GetOSRelease")]
    fn get_os_release(&self) -> crate::zbus::Result<::std::collections::HashMap<String, String>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetUIDShift()) Call interface method `GetUIDShift`.
    #[dbus_proxy(name = "GetUIDShift")]
    fn get_uid_shift(&self) -> crate::zbus::Result<u32>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenPTY()) Call interface method `OpenPTY`.
    #[dbus_proxy(name = "OpenPTY")]
    fn open_pty(&self) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenLogin()) Call interface method `OpenLogin`.
    #[dbus_proxy(name = "OpenLogin")]
    fn open_login(&self) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenShell()) Call interface method `OpenShell`.
    #[dbus_proxy(name = "OpenShell")]
    fn open_shell(
        &self,
        user: String,
        path: String,
        args: Vec<String>,
        environment: Vec<String>,
    ) -> crate::zbus::Result<(crate::zvariant::OwnedFd, String)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#BindMount()) Call interface method `BindMount`.
    #[dbus_proxy(name = "BindMount")]
    fn bind_mount(
        &self,
        source: String,
        destination: String,
        read_only: bool,
        mkdir: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyFrom()) Call interface method `CopyFrom`.
    #[dbus_proxy(name = "CopyFrom")]
    fn copy_from(&self, source: String, destination: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyTo()) Call interface method `CopyTo`.
    #[dbus_proxy(name = "CopyTo")]
    fn copy_to(&self, source: String, destination: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyFromWithFlags()) Call interface method `CopyFromWithFlags`.
    #[dbus_proxy(name = "CopyFromWithFlags")]
    fn copy_from_with_flags(
        &self,
        source: String,
        destination: String,
        flags: u64,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CopyToWithFlags()) Call interface method `CopyToWithFlags`.
    #[dbus_proxy(name = "CopyToWithFlags")]
    fn copy_to_with_flags(
        &self,
        source: String,
        destination: String,
        flags: u64,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#OpenRootDirectory()) Call interface method `OpenRootDirectory`.
    #[dbus_proxy(name = "OpenRootDirectory")]
    fn open_root_directory(&self) -> crate::zbus::Result<crate::zvariant::OwnedFd>;

    /// Get property `Name`.
    #[dbus_proxy(property, name = "Name")]
    fn name(&self) -> crate::zbus::Result<String>;

    /// Get property `Id`.
    #[dbus_proxy(property, name = "Id")]
    fn id(&self) -> crate::zbus::Result<Vec<u8>>;

    /// Get property `Timestamp`.
    #[dbus_proxy(property, name = "Timestamp")]
    fn timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `TimestampMonotonic`.
    #[dbus_proxy(property, name = "TimestampMonotonic")]
    fn timestamp_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `Service`.
    #[dbus_proxy(property, name = "Service")]
    fn service(&self) -> crate::zbus::Result<String>;

    /// Get property `Unit`.
    #[dbus_proxy(property, name = "Unit")]
    fn unit(&self) -> crate::zbus::Result<String>;

    /// Get property `Leader`.
    #[dbus_proxy(property, name = "Leader")]
    fn leader(&self) -> crate::zbus::Result<u32>;

    /// Get property `Class`.
    #[dbus_proxy(property, name = "Class")]
    fn class(&self) -> crate::zbus::Result<String>;

    /// Get property `RootDirectory`.
    #[dbus_proxy(property, name = "RootDirectory")]
    fn root_directory(&self) -> crate::zbus::Result<String>;

    /// Get property `NetworkInterfaces`.
    #[dbus_proxy(property, name = "NetworkInterfaces")]
    fn network_interfaces(&self) -> crate::zbus::Result<Vec<i32>>;

    /// Get property `State`.
    #[dbus_proxy(property, name = "State")]
    fn state(&self) -> crate::zbus::Result<String>;
}
