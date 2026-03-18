use crate::matcher::Matcher;
use matching_if::types::UserId;
use matching_if::webrtc::peer_connection_wrapper::PeerConnectionWrapper;
use matching_if::webrtc::web_rtc_wrapper::WebRtcWrapper;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

#[derive(Default)]
pub struct AppState {
    web_rtc_wrapper: WebRtcWrapper,
    matcher_to_wrappers: HashMap<Matcher, Vec<PeerConnectionWrapper>>,
    matcher_to_user_id: HashMap<Matcher, (UserId, Sender<(UserId, String)>)>,
}

impl AppState {
    pub fn get_web_rtc_wrapper(&self) -> &WebRtcWrapper {
        &self.web_rtc_wrapper
    }

    pub fn has_waiting_peer_connection_wrapper(&self, matcher: &Matcher) -> bool {
        self.matcher_to_wrappers.contains_key(matcher)
            && !self.matcher_to_wrappers.get(matcher).unwrap().is_empty()
    }

    pub fn has_waiting_user(&self, matcher: &Matcher) -> bool {
        self.matcher_to_user_id.contains_key(matcher)
    }

    pub fn clear_matcher_to_wrappers(&mut self) {
        self.matcher_to_wrappers.clear();
    }

    pub fn clear_matcher_to_user_id(&mut self) {
        self.matcher_to_user_id.clear();
    }
    pub fn get_waiting_user_id_from_wrappers(&self, matcher: &Matcher) -> Option<UserId> {
        if !self.has_waiting_peer_connection_wrapper(matcher) {
            return None;
        }
        self.matcher_to_wrappers
            .get(matcher)
            .unwrap()
            .iter()
            .map(|wrapper| *wrapper.get_user_id())
            .next()
    }

    pub fn get_waiting_user_id(
        &self,
        matcher: &Matcher,
    ) -> Option<(UserId, Sender<(UserId, String)>)> {
        self.matcher_to_user_id.get(matcher).cloned()
    }

    // TODO
    // waiting_user の複数化検討
    pub fn find_waiting_user_by_id(
        &mut self,
        matcher: &Matcher,
        user_id: &UserId,
    ) -> Option<(UserId, Sender<(UserId, String)>)> {
        let cloned = self.matcher_to_user_id.remove(matcher);
        if cloned.as_ref().is_some() && cloned.as_ref().unwrap().0 != *user_id {
            None
        } else {
            cloned
        }
    }
    pub fn insert_wrapper(&mut self, matcher: &Matcher, wrapper: PeerConnectionWrapper) {
        if self.matcher_to_wrappers.contains_key(matcher) {
            self.matcher_to_wrappers
                .get_mut(matcher)
                .unwrap()
                .push(wrapper);
        } else {
            self.matcher_to_wrappers
                .insert(matcher.clone(), vec![wrapper]);
        }
    }

    pub fn find_wrapper_by_user_id(
        &mut self,
        matcher: &Matcher,
        user_id: &UserId,
    ) -> Option<&mut PeerConnectionWrapper> {
        let wrappers = self.matcher_to_wrappers.get_mut(matcher);
        if wrappers.is_none() || wrappers.as_ref().unwrap().is_empty() {
            return None;
        }
        wrappers.into_iter().find_map(|wrappers| {
            wrappers
                .iter_mut()
                .find(|wrapper| wrapper.get_user_id() == user_id)
        })
    }

    pub fn remove_wrapper_by_user_id(
        &mut self,
        matcher: &Matcher,
        user_id: &UserId,
    ) -> Result<(), String> {
        let wrappers = self.matcher_to_wrappers.get_mut(matcher);
        let Some(wrappers) = wrappers else {
            return Err("matcher does not set".to_owned());
        };
        if wrappers.is_empty() {
            return Err("matcher has no wrappers".to_owned());
        }
        wrappers.retain(|wrapper| wrapper.get_user_id() != user_id);
        Ok(())
    }

    pub fn insert_waiting_user(
        &mut self,
        matcher: &Matcher,
        user_id: &UserId,
        sender: Sender<(UserId, String)>,
    ) {
        self.matcher_to_user_id
            .insert(matcher.clone(), (*user_id, sender));
    }
}
