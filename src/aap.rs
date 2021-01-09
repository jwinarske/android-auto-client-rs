use crate::aap::State::{Starting, Stopping};

mod head_unit;

#[derive(Debug)]
pub enum State {
    Initial,
    Starting,
    Started,
    Stopping,
    Stopped,
}
pub enum FrameFlags {
    FirstFrame = 1 << 0,
    LastFrame = 1 << 1,
    ControlMessage = 1 << 2,
    Encrypted = 1 << 3,
}
pub enum TransportType {
    USB,
    TCP,
}
struct TransportStream {
    read_fd: i32,
    error_fd: i32,
}

impl TransportStream {
    fn start(&self, wait_for_device: bool) -> u32 {
        return 0;
    }
    fn stop(&self, ) {
    }
    fn write(&self, buf: &[u8], len: isize, tmo: isize) {
    }
    fn get_read_fd(&self) -> i32 {
        return self.read_fd;
    }
    fn get_error_fd(&self) -> i32 {
        return self.error_fd;
    }
}

struct ConnectionThread {}

impl ConnectionThread {

    fn send_message(&self, retry: isize, chan: isize, message_code: u16, /* todo const google::protobuf::MessageLite& message,*/ override_timeout: isize) -> isize {
        0
    }
    fn send_message_enc(&self, retry: isize, chan: isize, message_code: u16, /* todo const google::protobuf::MessageLite& message,*/ override_timeout: isize) -> isize {
        0
    }
    fn send_media_packet_enc(&self, retry: isize, chan: isize, message_code: u16, time_stamp: u64, buffer: &[u8], override_timeout: isize) -> isize {
        0
    }
    fn send_blob(&self, retry: isize, chan: isize, message_code: u16, buffer: &[u8], override_timeout: isize) -> isize {
        0
    }
    fn stop(&self, ) -> isize {
        0
    }
}

struct EventCallbacks {}

impl EventCallbacks {
    fn new() -> EventCallbacks {
        return EventCallbacks {};
    }
    fn message_filter(&self, stream: &ConnectionThread, state: State, channel: isize, msg_type: u16, buf: &[u8]) -> isize {
        0
    }
    fn media_packet(&self, channel: isize, timestamp: u64, buf: &[u8]) -> isize {
        0
    }
    fn media_start(&self, channel: isize) -> isize {
        0
    }
    fn media_stop(&self, channel: isize) -> isize {
        0
    }
    fn media_setup_complete(&self, channel: isize) {

    }
    fn disconnection_or_error(&self, ) {

    }
    fn customize_car_info(&self, car_info: &head_unit::ServiceDiscoveryResponse) {

    }
//    fn customize_input_config(&self, input_channel: &<head_unit::ChannelDescriptor>::input_event_channel) {

//    }
//    fn customize_sensor_config(&self, sensor_channel: &<head_unit::ChannelDescriptor>::sensor_channel) {

//    }
//    fn customize_output_channel(&self, channel: isize, stream_channel: &<head_unit::ChannelDescriptor>::output_stream_channel) {

//    }
//    fn customize_input_channel(&self, channel: isize, stream_channel: &<head_unit::ChannelDescriptor>::input_stream_channel) {

//    }
//    fn customize_bluetooth_service(&self, channel: isize, bluetooth_service: &<head_unit::ChannelDescriptor>::bluetooth_service) {

//    }
    fn get_car_bluetooth_address() -> String {
        return "".parse().unwrap()
    }
    fn audio_focus_request(channel: isize, request: &head_unit::AudioFocusRequest) {

    }
    fn video_focus_request(channel: isize, request: &head_unit::VideoFocusRequest) {

    }
    fn handle_phone_status(stream: &ConnectionThread, phone_status: &head_unit::PhoneStatus) {

    }
    fn handle_generic_notification_response(stream: &ConnectionThread, response: &head_unit::GenericNotificationResponse) {

    }
    fn showing_generic_notifications(stream: &ConnectionThread, is_showing: bool) {

    }
    fn handle_navi_status(stream: &ConnectionThread, request: &head_unit::NAVMessagesStatus) {

    }
    fn handle_navi_turn(stream: &ConnectionThread, request: &head_unit::NAVTurnMessage) {

    }
    fn handle_navi_turn_distance(stream: &ConnectionThread, request: &head_unit::NAVDistanceMessage) {

    }
}

pub struct Server {
    callbacks: EventCallbacks,
    state: State,
}

impl Server {
    pub fn new() -> Server {
        return Server{
            callbacks: EventCallbacks {},
            state: State::Initial
        }
    }
    pub fn start (&mut self, transport_type: &TransportType, phone_ip_address: &str) {
        self.state = State::Starting;
        self.state = State::Started;
        debug!("state: {:?}", &self.state);
    }
    pub fn shutdown(&mut self, ) {
        self.state = State::Stopping;
        self.state = State::Stopped;
        debug!("state: {:?}", &self.state);
    }
    pub fn state(&self) -> &State {
        return &self.state;
    }
}
