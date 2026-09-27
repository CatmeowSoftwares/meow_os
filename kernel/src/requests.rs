use limine::{
    BaseRevision, RequestsEndMarker, RequestsStartMarker,
    request::{
        ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemmapRequest, RsdpRequest,
        TscFrequencyRequest,
    },
};
#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EXECUTABLE_ADDRESS_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static TSC_FREQUENCY_REQUEST: TscFrequencyRequest = TscFrequencyRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".limine_requests_end")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();
