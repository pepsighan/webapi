#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type HTMLButtonElement;

    pub type HTMLProgressElement;

    pub type SharedWorker;

    pub type Element;

    pub type HTMLHRElement;

    pub type AbstractRange;

    pub type HTMLMenuElement;

    pub type HTMLLabelElement;

    pub type HTMLTimeElement;

    pub type TextTrackCueList;

    pub type PopStateEvent;

    pub type HTMLTemplateElement;

    pub type MessagePort;

    pub type NodeList;

    pub type HTMLFormElement;

    pub type HTMLDetailsElement;

    pub type Location;

    pub type TreeWalker;

    pub type BeforeUnloadEvent;

    pub type CDATASection;

    pub type TextMetrics;

    pub type HTMLSourceElement;

    pub type MutationRecord;

    pub type HTMLBodyElement;

    pub type TimeRanges;

    pub type HTMLUnknownElement;

    pub type HTMLAnchorElement;

    pub type DataTransferItem;

    pub type WorkerLocation;

    pub type AudioTrackList;

    pub type HTMLHeadingElement;

    pub type HTMLFrameElement;

    pub type HTMLPreElement;

    pub type HTMLMediaElement;

    pub type HTMLEmbedElement;

    pub type DataTransferItemList;

    pub type HTMLLIElement;

    pub type HTMLStyleElement;

    pub type HTMLTableCaptionElement;

    pub type HTMLOptionElement;

    pub type ValidityState;

    pub type WorkerNavigator;

    pub type HTMLQuoteElement;

    pub type HTMLParagraphElement;

    pub type ImageBitmapRenderingContext;

    pub type OffscreenCanvas;

    pub type HTMLMeterElement;

    pub type HTMLUListElement;

    pub type VideoTrackList;

    pub type HTMLSelectElement;

    pub type CanvasPattern;

    pub type HTMLObjectElement;

    pub type HTMLDialogElement;

    pub type DataTransfer;

    pub type HTMLScriptElement;

    pub type ApplicationCache;

    pub type Plugin;

    pub type HTMLModElement;

    pub type ImageData;

    pub type window;

    pub type HTMLLinkElement;

    pub type WebSocket;

    pub type HTMLImageElement;

    pub type HTMLTextAreaElement;

    pub type CharacterData;

    pub type HTMLParamElement;

    pub type PageTransitionEvent;

    pub type HTMLTableRowElement;

    pub type EventSource;

    pub type HTMLInputElement;

    pub type DocumentType;

    pub type HTMLCollection;

    pub type MessageChannel;

    pub type HTMLTableCellElement;

    pub type DOMImplementation;

    pub type HTMLLegendElement;

    pub type PluginArray;

    pub type HTMLOutputElement;

    pub type DOMStringList;

    pub type MessageEvent;

    pub type HTMLVideoElement;

    pub type Attr;

    pub type MimeTypeArray;

    pub type HTMLAreaElement;

    pub type TextTrackCue;

    pub type HTMLTrackElement;

    pub type HTMLMarqueeElement;

    pub type Node;

    pub type HTMLElement;

    pub type HTMLTableSectionElement;

    pub type HTMLPictureElement;

    pub type AudioTrack;

    pub type HTMLSlotElement;

    pub type DocumentFragment;

    pub type HTMLTableElement;

    pub type RadioNodeList;

    pub type HTMLMapElement;

    pub type HTMLDListElement;

    pub type StorageEvent;

    pub type Storage;

    pub type HTMLDataListElement;

    pub type CustomElementRegistry;

    pub type HTMLAudioElement;

    pub type CloseEvent;

    pub type HTMLDirectoryElement;

    pub type CustomEvent;

    pub type HTMLFieldSetElement;

    pub type HTMLDivElement;

    pub type HTMLSpanElement;

    pub type StaticRange;

    pub type HTMLDataElement;

    pub type NodeIterator;

    pub type HTMLHtmlElement;

    pub type HTMLOptionsCollection;

    pub type EventTarget;

    pub type CanvasRenderingContext2D;

    pub type PromiseRejectionEvent;

    pub type Document;

    pub type OffscreenCanvasRenderingContext2D;

    pub type HTMLHeadElement;

    pub type DragEvent;

    pub type HTMLFormControlsCollection;

    pub type SharedWorkerGlobalScope;

    pub type AbortController;

    pub type ShadowRoot;

    pub type DOMTokenList;

    pub type AbortSignal;

    pub type Worker;

    pub type Path2D;

    pub type HashChangeEvent;

    pub type MediaError;

    pub type TrackEvent;

    pub type HTMLCanvasElement;

    pub type CanvasGradient;

    pub type DOMStringMap;

    pub type Navigator;

    pub type MutationObserver;

    pub type BarProp;

    pub type ErrorEvent;

    pub type HTMLTableColElement;

    pub type History;

    pub type ImageBitmap;

    pub type DedicatedWorkerGlobalScope;

    pub type Text;

    pub type ProcessingInstruction;

    pub type HTMLBaseElement;

    pub type HTMLFrameSetElement;

    pub type XMLDocument;

    pub type Comment;

    pub type HTMLIFrameElement;

    pub type HTMLOListElement;

    pub type BroadcastChannel;

    pub type Range;

    pub type Event;

    pub type NamedNodeMap;

    pub type HTMLFontElement;

    pub type WorkerGlobalScope;

    pub type VideoTrack;

    pub type TextTrack;

    pub type TextTrackList;

    pub type HTMLOptGroupElement;

    pub type HTMLAllCollection;

    pub type HTMLBRElement;

    pub type HTMLMetaElement;

    pub type MimeType;

    pub type HTMLTitleElement;

    #[wasm_bindgen(method, getter, structural)]
    pub fn signal(this: &AbortController) -> AbortSignal;

    #[wasm_bindgen(method, getter, structural)]
    pub fn aborted(this: &AbortSignal) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn collapsed(this: &AbstractRange) -> bool;

    #[wasm_bindgen(method, getter = endContainer, structural)]
    pub fn end_container(this: &AbstractRange) -> Node;

    #[wasm_bindgen(method, getter = endOffset, structural)]
    pub fn end_offset(this: &AbstractRange) -> u32;

    #[wasm_bindgen(method, getter = startContainer, structural)]
    pub fn start_container(this: &AbstractRange) -> Node;

    #[wasm_bindgen(method, getter = startOffset, structural)]
    pub fn start_offset(this: &AbstractRange) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn status(this: &ApplicationCache) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &Attr) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &Attr, val: String);

    #[wasm_bindgen(method, getter = localName, structural)]
    pub fn local_name(this: &Attr) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &Attr) -> String;

    #[wasm_bindgen(method, getter = namespaceURI, structural)]
    pub fn namespace_uri(this: &Attr) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn prefix(this: &Attr) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn specified(this: &Attr) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn enabled(this: &AudioTrack) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_enabled(this: &AudioTrack, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &AudioTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn kind(this: &AudioTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &AudioTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn language(this: &AudioTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &AudioTrackList) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn visible(this: &BarProp) -> bool;

    #[wasm_bindgen(method, getter = returnValue, structural)]
    pub fn return_value(this: &BeforeUnloadEvent) -> String;

    #[wasm_bindgen(method, setter = returnValue, structural)]
    pub fn set_return_value(this: &BeforeUnloadEvent, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &BroadcastChannel) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn canvas(this: &CanvasRenderingContext2D) -> HTMLCanvasElement;

    #[wasm_bindgen(method, getter, structural)]
    pub fn data(this: &CharacterData) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_data(this: &CharacterData, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &CharacterData) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn code(this: &CloseEvent) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn reason(this: &CloseEvent) -> String;

    #[wasm_bindgen(method, getter = wasClean, structural)]
    pub fn was_clean(this: &CloseEvent) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &DOMStringList) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &DOMTokenList) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &DOMTokenList, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &DOMTokenList) -> u32;

    #[wasm_bindgen(method, getter = dropEffect, structural)]
    pub fn drop_effect(this: &DataTransfer) -> String;

    #[wasm_bindgen(method, setter = dropEffect, structural)]
    pub fn set_drop_effect(this: &DataTransfer, val: String);

    #[wasm_bindgen(method, getter = effectAllowed, structural)]
    pub fn effect_allowed(this: &DataTransfer) -> String;

    #[wasm_bindgen(method, setter = effectAllowed, structural)]
    pub fn set_effect_allowed(this: &DataTransfer, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn items(this: &DataTransfer) -> DataTransferItemList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn kind(this: &DataTransferItem) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &DataTransferItemList) -> u32;

    #[wasm_bindgen(method, getter = URL, structural)]
    pub fn url(this: &Document) -> String;

    #[wasm_bindgen(method, getter = characterSet, structural)]
    pub fn character_set(this: &Document) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn charset(this: &Document) -> String;

    #[wasm_bindgen(method, getter = compatMode, structural)]
    pub fn compat_mode(this: &Document) -> String;

    #[wasm_bindgen(method, getter = contentType, structural)]
    pub fn content_type(this: &Document) -> String;

    #[wasm_bindgen(method, getter = documentURI, structural)]
    pub fn document_uri(this: &Document) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn implementation(this: &Document) -> DOMImplementation;

    #[wasm_bindgen(method, getter = inputEncoding, structural)]
    pub fn input_encoding(this: &Document) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &Document) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &DocumentType) -> String;

    #[wasm_bindgen(method, getter = publicId, structural)]
    pub fn public_id(this: &DocumentType) -> String;

    #[wasm_bindgen(method, getter = systemId, structural)]
    pub fn system_id(this: &DocumentType) -> String;

    #[wasm_bindgen(method, getter = className, structural)]
    pub fn class_name(this: &Element) -> String;

    #[wasm_bindgen(method, setter = className, structural)]
    pub fn set_class_name(this: &Element, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &Element) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_id(this: &Element, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn slot(this: &Element) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_slot(this: &Element, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn attributes(this: &Element) -> NamedNodeMap;

    #[wasm_bindgen(method, getter = classList, structural)]
    pub fn class_list(this: &Element) -> DOMTokenList;

    #[wasm_bindgen(method, getter = localName, structural)]
    pub fn local_name(this: &Element) -> String;

    #[wasm_bindgen(method, getter = namespaceURI, structural)]
    pub fn namespace_uri(this: &Element) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn prefix(this: &Element) -> String;

    #[wasm_bindgen(method, getter = tagName, structural)]
    pub fn tag_name(this: &Element) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn colno(this: &ErrorEvent) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn filename(this: &ErrorEvent) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn lineno(this: &ErrorEvent) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn message(this: &ErrorEvent) -> String;

    #[wasm_bindgen(method, getter = cancelBubble, structural)]
    pub fn cancel_bubble(this: &Event) -> bool;

    #[wasm_bindgen(method, setter = cancelBubble, structural)]
    pub fn set_cancel_bubble(this: &Event, val: bool);

    #[wasm_bindgen(method, getter = returnValue, structural)]
    pub fn return_value(this: &Event) -> bool;

    #[wasm_bindgen(method, setter = returnValue, structural)]
    pub fn set_return_value(this: &Event, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn bubbles(this: &Event) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn cancelable(this: &Event) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn composed(this: &Event) -> bool;

    #[wasm_bindgen(method, getter = defaultPrevented, structural)]
    pub fn default_prevented(this: &Event) -> bool;

    #[wasm_bindgen(method, getter = eventPhase, structural)]
    pub fn event_phase(this: &Event) -> u16;

    #[wasm_bindgen(method, getter = isTrusted, structural)]
    pub fn is_trusted(this: &Event) -> bool;

    #[wasm_bindgen(method, getter = readyState, structural)]
    pub fn ready_state(this: &EventSource) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn url(this: &EventSource) -> String;

    #[wasm_bindgen(method, getter = withCredentials, structural)]
    pub fn with_credentials(this: &EventSource) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &HTMLAllCollection) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn download(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_download(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn hreflang(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hreflang(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn ping(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_ping(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter = referrerPolicy, structural)]
    pub fn referrer_policy(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter = referrerPolicy, structural)]
    pub fn set_referrer_policy(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rel(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_rel(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_target(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn text(this: &HTMLAnchorElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_text(this: &HTMLAnchorElement, val: String);

    #[wasm_bindgen(method, getter = relList, structural)]
    pub fn rel_list(this: &HTMLAnchorElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn alt(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_alt(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn coords(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_coords(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn download(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_download(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn ping(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_ping(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter = referrerPolicy, structural)]
    pub fn referrer_policy(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter = referrerPolicy, structural)]
    pub fn set_referrer_policy(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rel(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_rel(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn shape(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_shape(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &HTMLAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_target(this: &HTMLAreaElement, val: String);

    #[wasm_bindgen(method, getter = relList, structural)]
    pub fn rel_list(this: &HTMLAreaElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn href(this: &HTMLBaseElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_href(this: &HTMLBaseElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &HTMLBaseElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_target(this: &HTMLBaseElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autofocus(this: &HTMLButtonElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autofocus(this: &HTMLButtonElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLButtonElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLButtonElement, val: bool);

    #[wasm_bindgen(method, getter = formAction, structural)]
    pub fn form_action(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter = formAction, structural)]
    pub fn set_form_action(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter = formEnctype, structural)]
    pub fn form_enctype(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter = formEnctype, structural)]
    pub fn set_form_enctype(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter = formMethod, structural)]
    pub fn form_method(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter = formMethod, structural)]
    pub fn set_form_method(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter = formNoValidate, structural)]
    pub fn form_no_validate(this: &HTMLButtonElement) -> bool;

    #[wasm_bindgen(method, setter = formNoValidate, structural)]
    pub fn set_form_no_validate(this: &HTMLButtonElement, val: bool);

    #[wasm_bindgen(method, getter = formTarget, structural)]
    pub fn form_target(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter = formTarget, structural)]
    pub fn set_form_target(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLButtonElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLButtonElement) -> NodeList;

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLButtonElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLButtonElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLButtonElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLCanvasElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLCanvasElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &HTMLCollection) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLDataElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLDataElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn options(this: &HTMLDataListElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter, structural)]
    pub fn open(this: &HTMLDetailsElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_open(this: &HTMLDetailsElement, val: bool);

    #[wasm_bindgen(method, getter = returnValue, structural)]
    pub fn return_value(this: &HTMLDialogElement) -> String;

    #[wasm_bindgen(method, setter = returnValue, structural)]
    pub fn set_return_value(this: &HTMLDialogElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn compact(this: &HTMLDirectoryElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_compact(this: &HTMLDirectoryElement, val: bool);

    #[wasm_bindgen(method, getter = accessKey, structural)]
    pub fn access_key(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter = accessKey, structural)]
    pub fn set_access_key(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autocapitalize(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autocapitalize(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn dir(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_dir(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn draggable(this: &HTMLElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_draggable(this: &HTMLElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn hidden(this: &HTMLElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hidden(this: &HTMLElement, val: bool);

    #[wasm_bindgen(method, getter = innerText, structural)]
    pub fn inner_text(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter = innerText, structural)]
    pub fn set_inner_text(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn lang(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_lang(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn spellcheck(this: &HTMLElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_spellcheck(this: &HTMLElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn title(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_title(this: &HTMLElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn translate(this: &HTMLElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_translate(this: &HTMLElement, val: bool);

    #[wasm_bindgen(method, getter = accessKeyLabel, structural)]
    pub fn access_key_label(this: &HTMLElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLEmbedElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLEmbedElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLEmbedElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLEmbedElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLEmbedElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLEmbedElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLFieldSetElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLFieldSetElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLFieldSetElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLFieldSetElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn elements(this: &HTMLFieldSetElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLFieldSetElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLFieldSetElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLFieldSetElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn color(this: &HTMLFontElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_color(this: &HTMLFontElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn face(this: &HTMLFontElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_face(this: &HTMLFontElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &HTMLFontElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_size(this: &HTMLFontElement, val: String);

    #[wasm_bindgen(method, getter = acceptCharset, structural)]
    pub fn accept_charset(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter = acceptCharset, structural)]
    pub fn set_accept_charset(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn action(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_action(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autocomplete(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autocomplete(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn encoding(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_encoding(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn enctype(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_enctype(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn method(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_method(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter = noValidate, structural)]
    pub fn no_validate(this: &HTMLFormElement) -> bool;

    #[wasm_bindgen(method, setter = noValidate, structural)]
    pub fn set_no_validate(this: &HTMLFormElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &HTMLFormElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_target(this: &HTMLFormElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn elements(this: &HTMLFormElement) -> HTMLFormControlsCollection;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &HTMLFormElement) -> u32;

    #[wasm_bindgen(method, getter = frameBorder, structural)]
    pub fn frame_border(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter = frameBorder, structural)]
    pub fn set_frame_border(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter = longDesc, structural)]
    pub fn long_desc(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter = longDesc, structural)]
    pub fn set_long_desc(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter = marginHeight, structural)]
    pub fn margin_height(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter = marginHeight, structural)]
    pub fn set_margin_height(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter = marginWidth, structural)]
    pub fn margin_width(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter = marginWidth, structural)]
    pub fn set_margin_width(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter = noResize, structural)]
    pub fn no_resize(this: &HTMLFrameElement) -> bool;

    #[wasm_bindgen(method, setter = noResize, structural)]
    pub fn set_no_resize(this: &HTMLFrameElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn scrolling(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_scrolling(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn cols(this: &HTMLFrameSetElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_cols(this: &HTMLFrameSetElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rows(this: &HTMLFrameSetElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_rows(this: &HTMLFrameSetElement, val: String);

    #[wasm_bindgen(method, getter = allowFullscreen, structural)]
    pub fn allow_fullscreen(this: &HTMLIFrameElement) -> bool;

    #[wasm_bindgen(method, setter = allowFullscreen, structural)]
    pub fn set_allow_fullscreen(this: &HTMLIFrameElement, val: bool);

    #[wasm_bindgen(method, getter = allowPaymentRequest, structural)]
    pub fn allow_payment_request(this: &HTMLIFrameElement) -> bool;

    #[wasm_bindgen(method, setter = allowPaymentRequest, structural)]
    pub fn set_allow_payment_request(this: &HTMLIFrameElement, val: bool);

    #[wasm_bindgen(method, getter = allowUserMedia, structural)]
    pub fn allow_user_media(this: &HTMLIFrameElement) -> bool;

    #[wasm_bindgen(method, setter = allowUserMedia, structural)]
    pub fn set_allow_user_media(this: &HTMLIFrameElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter = referrerPolicy, structural)]
    pub fn referrer_policy(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter = referrerPolicy, structural)]
    pub fn set_referrer_policy(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn srcdoc(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_srcdoc(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLIFrameElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLIFrameElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn sandbox(this: &HTMLIFrameElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn alt(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_alt(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter = crossOrigin, structural)]
    pub fn cross_origin(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter = crossOrigin, structural)]
    pub fn set_cross_origin(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn decoding(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_decoding(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLImageElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLImageElement, val: u32);

    #[wasm_bindgen(method, getter = isMap, structural)]
    pub fn is_map(this: &HTMLImageElement) -> bool;

    #[wasm_bindgen(method, setter = isMap, structural)]
    pub fn set_is_map(this: &HTMLImageElement, val: bool);

    #[wasm_bindgen(method, getter = referrerPolicy, structural)]
    pub fn referrer_policy(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter = referrerPolicy, structural)]
    pub fn set_referrer_policy(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn sizes(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_sizes(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn srcset(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_srcset(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter = useMap, structural)]
    pub fn use_map(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, setter = useMap, structural)]
    pub fn set_use_map(this: &HTMLImageElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLImageElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLImageElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn complete(this: &HTMLImageElement) -> bool;

    #[wasm_bindgen(method, getter = currentSrc, structural)]
    pub fn current_src(this: &HTMLImageElement) -> String;

    #[wasm_bindgen(method, getter = naturalHeight, structural)]
    pub fn natural_height(this: &HTMLImageElement) -> u32;

    #[wasm_bindgen(method, getter = naturalWidth, structural)]
    pub fn natural_width(this: &HTMLImageElement) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn accept(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_accept(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn alt(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_alt(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autocomplete(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autocomplete(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autofocus(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autofocus(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn checked(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_checked(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter = defaultChecked, structural)]
    pub fn default_checked(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter = defaultChecked, structural)]
    pub fn set_default_checked(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter = defaultValue, structural)]
    pub fn default_value(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = defaultValue, structural)]
    pub fn set_default_value(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = dirName, structural)]
    pub fn dir_name(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = dirName, structural)]
    pub fn set_dir_name(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter = formAction, structural)]
    pub fn form_action(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = formAction, structural)]
    pub fn set_form_action(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = formEnctype, structural)]
    pub fn form_enctype(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = formEnctype, structural)]
    pub fn set_form_enctype(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = formMethod, structural)]
    pub fn form_method(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = formMethod, structural)]
    pub fn set_form_method(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = formNoValidate, structural)]
    pub fn form_no_validate(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter = formNoValidate, structural)]
    pub fn set_form_no_validate(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter = formTarget, structural)]
    pub fn form_target(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = formTarget, structural)]
    pub fn set_form_target(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLInputElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLInputElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn indeterminate(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_indeterminate(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn max(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_max(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = maxLength, structural)]
    pub fn max_length(this: &HTMLInputElement) -> i32;

    #[wasm_bindgen(method, setter = maxLength, structural)]
    pub fn set_max_length(this: &HTMLInputElement, val: i32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn min(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_min(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = minLength, structural)]
    pub fn min_length(this: &HTMLInputElement) -> i32;

    #[wasm_bindgen(method, setter = minLength, structural)]
    pub fn set_min_length(this: &HTMLInputElement, val: i32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn multiple(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_multiple(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn pattern(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_pattern(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn placeholder(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_placeholder(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = readOnly, structural)]
    pub fn read_only(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter = readOnly, structural)]
    pub fn set_read_only(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn required(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_required(this: &HTMLInputElement, val: bool);

    #[wasm_bindgen(method, getter = selectionDirection, structural)]
    pub fn selection_direction(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = selectionDirection, structural)]
    pub fn set_selection_direction(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = selectionEnd, structural)]
    pub fn selection_end(this: &HTMLInputElement) -> u32;

    #[wasm_bindgen(method, setter = selectionEnd, structural)]
    pub fn set_selection_end(this: &HTMLInputElement, val: u32);

    #[wasm_bindgen(method, getter = selectionStart, structural)]
    pub fn selection_start(this: &HTMLInputElement) -> u32;

    #[wasm_bindgen(method, setter = selectionStart, structural)]
    pub fn set_selection_start(this: &HTMLInputElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &HTMLInputElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_size(this: &HTMLInputElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn step(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_step(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLInputElement, val: String);

    #[wasm_bindgen(method, getter = valueAsNumber, structural)]
    pub fn value_as_number(this: &HTMLInputElement) -> f64;

    #[wasm_bindgen(method, setter = valueAsNumber, structural)]
    pub fn set_value_as_number(this: &HTMLInputElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLInputElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLInputElement, val: u32);

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLInputElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLInputElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLLIElement) -> i32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLLIElement, val: i32);

    #[wasm_bindgen(method, getter = htmlFor, structural)]
    pub fn html_for(this: &HTMLLabelElement) -> String;

    #[wasm_bindgen(method, setter = htmlFor, structural)]
    pub fn set_html_for(this: &HTMLLabelElement, val: String);

    #[wasm_bindgen(method, getter = crossOrigin, structural)]
    pub fn cross_origin(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter = crossOrigin, structural)]
    pub fn set_cross_origin(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn href(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_href(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn hreflang(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hreflang(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn integrity(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_integrity(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn media(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_media(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter = referrerPolicy, structural)]
    pub fn referrer_policy(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter = referrerPolicy, structural)]
    pub fn set_referrer_policy(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rel(this: &HTMLLinkElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_rel(this: &HTMLLinkElement, val: String);

    #[wasm_bindgen(method, getter = relList, structural)]
    pub fn rel_list(this: &HTMLLinkElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn sizes(this: &HTMLLinkElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLMapElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLMapElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn areas(this: &HTMLMapElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter, structural)]
    pub fn behavior(this: &HTMLMarqueeElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_behavior(this: &HTMLMarqueeElement, val: String);

    #[wasm_bindgen(method, getter = bgColor, structural)]
    pub fn bg_color(this: &HTMLMarqueeElement) -> String;

    #[wasm_bindgen(method, setter = bgColor, structural)]
    pub fn set_bg_color(this: &HTMLMarqueeElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn direction(this: &HTMLMarqueeElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_direction(this: &HTMLMarqueeElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLMarqueeElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLMarqueeElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn hspace(this: &HTMLMarqueeElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hspace(this: &HTMLMarqueeElement, val: u32);

    #[wasm_bindgen(method, getter = scrollAmount, structural)]
    pub fn scroll_amount(this: &HTMLMarqueeElement) -> u32;

    #[wasm_bindgen(method, setter = scrollAmount, structural)]
    pub fn set_scroll_amount(this: &HTMLMarqueeElement, val: u32);

    #[wasm_bindgen(method, getter = scrollDelay, structural)]
    pub fn scroll_delay(this: &HTMLMarqueeElement) -> u32;

    #[wasm_bindgen(method, setter = scrollDelay, structural)]
    pub fn set_scroll_delay(this: &HTMLMarqueeElement, val: u32);

    #[wasm_bindgen(method, getter = trueSpeed, structural)]
    pub fn true_speed(this: &HTMLMarqueeElement) -> bool;

    #[wasm_bindgen(method, setter = trueSpeed, structural)]
    pub fn set_true_speed(this: &HTMLMarqueeElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn vspace(this: &HTMLMarqueeElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_vspace(this: &HTMLMarqueeElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLMarqueeElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLMarqueeElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autoplay(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autoplay(this: &HTMLMediaElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn controls(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_controls(this: &HTMLMediaElement, val: bool);

    #[wasm_bindgen(method, getter = crossOrigin, structural)]
    pub fn cross_origin(this: &HTMLMediaElement) -> String;

    #[wasm_bindgen(method, setter = crossOrigin, structural)]
    pub fn set_cross_origin(this: &HTMLMediaElement, val: String);

    #[wasm_bindgen(method, getter = currentTime, structural)]
    pub fn current_time(this: &HTMLMediaElement) -> f64;

    #[wasm_bindgen(method, setter = currentTime, structural)]
    pub fn set_current_time(this: &HTMLMediaElement, val: f64);

    #[wasm_bindgen(method, getter = defaultMuted, structural)]
    pub fn default_muted(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, setter = defaultMuted, structural)]
    pub fn set_default_muted(this: &HTMLMediaElement, val: bool);

    #[wasm_bindgen(method, getter = defaultPlaybackRate, structural)]
    pub fn default_playback_rate(this: &HTMLMediaElement) -> f64;

    #[wasm_bindgen(method, setter = defaultPlaybackRate, structural)]
    pub fn set_default_playback_rate(this: &HTMLMediaElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn muted(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_muted(this: &HTMLMediaElement, val: bool);

    #[wasm_bindgen(method, getter = playbackRate, structural)]
    pub fn playback_rate(this: &HTMLMediaElement) -> f64;

    #[wasm_bindgen(method, setter = playbackRate, structural)]
    pub fn set_playback_rate(this: &HTMLMediaElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn preload(this: &HTMLMediaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_preload(this: &HTMLMediaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLMediaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLMediaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn volume(this: &HTMLMediaElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_volume(this: &HTMLMediaElement, val: f64);

    #[wasm_bindgen(method, getter = audioTracks, structural)]
    pub fn audio_tracks(this: &HTMLMediaElement) -> AudioTrackList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn buffered(this: &HTMLMediaElement) -> TimeRanges;

    #[wasm_bindgen(method, getter = currentSrc, structural)]
    pub fn current_src(this: &HTMLMediaElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn duration(this: &HTMLMediaElement) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn ended(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, getter = networkState, structural)]
    pub fn network_state(this: &HTMLMediaElement) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn paused(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn played(this: &HTMLMediaElement) -> TimeRanges;

    #[wasm_bindgen(method, getter = readyState, structural)]
    pub fn ready_state(this: &HTMLMediaElement) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn seekable(this: &HTMLMediaElement) -> TimeRanges;

    #[wasm_bindgen(method, getter, structural)]
    pub fn seeking(this: &HTMLMediaElement) -> bool;

    #[wasm_bindgen(method, getter = textTracks, structural)]
    pub fn text_tracks(this: &HTMLMediaElement) -> TextTrackList;

    #[wasm_bindgen(method, getter = videoTracks, structural)]
    pub fn video_tracks(this: &HTMLMediaElement) -> VideoTrackList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn content(this: &HTMLMetaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_content(this: &HTMLMetaElement, val: String);

    #[wasm_bindgen(method, getter = httpEquiv, structural)]
    pub fn http_equiv(this: &HTMLMetaElement) -> String;

    #[wasm_bindgen(method, setter = httpEquiv, structural)]
    pub fn set_http_equiv(this: &HTMLMetaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLMetaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLMetaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn high(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_high(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn low(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_low(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn max(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_max(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn min(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_min(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn optimum(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_optimum(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLMeterElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLMeterElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLMeterElement) -> NodeList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn cite(this: &HTMLModElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_cite(this: &HTMLModElement, val: String);

    #[wasm_bindgen(method, getter = dateTime, structural)]
    pub fn date_time(this: &HTMLModElement) -> String;

    #[wasm_bindgen(method, setter = dateTime, structural)]
    pub fn set_date_time(this: &HTMLModElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn reversed(this: &HTMLOListElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_reversed(this: &HTMLOListElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn start(this: &HTMLOListElement) -> i32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_start(this: &HTMLOListElement, val: i32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn data(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_data(this: &HTMLObjectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLObjectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLObjectElement, val: String);

    #[wasm_bindgen(method, getter = typeMustMatch, structural)]
    pub fn type_must_match(this: &HTMLObjectElement) -> bool;

    #[wasm_bindgen(method, setter = typeMustMatch, structural)]
    pub fn set_type_must_match(this: &HTMLObjectElement, val: bool);

    #[wasm_bindgen(method, getter = useMap, structural)]
    pub fn use_map(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, setter = useMap, structural)]
    pub fn set_use_map(this: &HTMLObjectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLObjectElement, val: String);

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLObjectElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLObjectElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLObjectElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLOptGroupElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLOptGroupElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &HTMLOptGroupElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_label(this: &HTMLOptGroupElement, val: String);

    #[wasm_bindgen(method, getter = defaultSelected, structural)]
    pub fn default_selected(this: &HTMLOptionElement) -> bool;

    #[wasm_bindgen(method, setter = defaultSelected, structural)]
    pub fn set_default_selected(this: &HTMLOptionElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLOptionElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLOptionElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &HTMLOptionElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_label(this: &HTMLOptionElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn selected(this: &HTMLOptionElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_selected(this: &HTMLOptionElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn text(this: &HTMLOptionElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_text(this: &HTMLOptionElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLOptionElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLOptionElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn index(this: &HTMLOptionElement) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &HTMLOptionsCollection) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_length(this: &HTMLOptionsCollection, val: u32);

    #[wasm_bindgen(method, getter = selectedIndex, structural)]
    pub fn selected_index(this: &HTMLOptionsCollection) -> i32;

    #[wasm_bindgen(method, setter = selectedIndex, structural)]
    pub fn set_selected_index(this: &HTMLOptionsCollection, val: i32);

    #[wasm_bindgen(method, getter = defaultValue, structural)]
    pub fn default_value(this: &HTMLOutputElement) -> String;

    #[wasm_bindgen(method, setter = defaultValue, structural)]
    pub fn set_default_value(this: &HTMLOutputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLOutputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLOutputElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLOutputElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLOutputElement, val: String);

    #[wasm_bindgen(method, getter = htmlFor, structural)]
    pub fn html_for(this: &HTMLOutputElement) -> DOMTokenList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLOutputElement) -> NodeList;

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLOutputElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLOutputElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLOutputElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLParamElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLParamElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLParamElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLParamElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn max(this: &HTMLProgressElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_max(this: &HTMLProgressElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLProgressElement) -> f64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLProgressElement, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLProgressElement) -> NodeList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn position(this: &HTMLProgressElement) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn cite(this: &HTMLQuoteElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_cite(this: &HTMLQuoteElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn async(this: &HTMLScriptElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_async(this: &HTMLScriptElement, val: bool);

    #[wasm_bindgen(method, getter = crossOrigin, structural)]
    pub fn cross_origin(this: &HTMLScriptElement) -> String;

    #[wasm_bindgen(method, setter = crossOrigin, structural)]
    pub fn set_cross_origin(this: &HTMLScriptElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn defer(this: &HTMLScriptElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_defer(this: &HTMLScriptElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn integrity(this: &HTMLScriptElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_integrity(this: &HTMLScriptElement, val: String);

    #[wasm_bindgen(method, getter = noModule, structural)]
    pub fn no_module(this: &HTMLScriptElement) -> bool;

    #[wasm_bindgen(method, setter = noModule, structural)]
    pub fn set_no_module(this: &HTMLScriptElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLScriptElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLScriptElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn text(this: &HTMLScriptElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_text(this: &HTMLScriptElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autocomplete(this: &HTMLSelectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autocomplete(this: &HTMLSelectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autofocus(this: &HTMLSelectElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autofocus(this: &HTMLSelectElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLSelectElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLSelectElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &HTMLSelectElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_length(this: &HTMLSelectElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn multiple(this: &HTMLSelectElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_multiple(this: &HTMLSelectElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLSelectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLSelectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn required(this: &HTMLSelectElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_required(this: &HTMLSelectElement, val: bool);

    #[wasm_bindgen(method, getter = selectedIndex, structural)]
    pub fn selected_index(this: &HTMLSelectElement) -> i32;

    #[wasm_bindgen(method, setter = selectedIndex, structural)]
    pub fn set_selected_index(this: &HTMLSelectElement, val: i32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn size(this: &HTMLSelectElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_size(this: &HTMLSelectElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLSelectElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLSelectElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLSelectElement) -> NodeList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn options(this: &HTMLSelectElement) -> HTMLOptionsCollection;

    #[wasm_bindgen(method, getter = selectedOptions, structural)]
    pub fn selected_options(this: &HTMLSelectElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLSelectElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLSelectElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLSelectElement) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLSlotElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLSlotElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn media(this: &HTMLSourceElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_media(this: &HTMLSourceElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn sizes(this: &HTMLSourceElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_sizes(this: &HTMLSourceElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLSourceElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLSourceElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn srcset(this: &HTMLSourceElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_srcset(this: &HTMLSourceElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn media(this: &HTMLStyleElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_media(this: &HTMLStyleElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn abbr(this: &HTMLTableCellElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_abbr(this: &HTMLTableCellElement, val: String);

    #[wasm_bindgen(method, getter = colSpan, structural)]
    pub fn col_span(this: &HTMLTableCellElement) -> u32;

    #[wasm_bindgen(method, setter = colSpan, structural)]
    pub fn set_col_span(this: &HTMLTableCellElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn headers(this: &HTMLTableCellElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_headers(this: &HTMLTableCellElement, val: String);

    #[wasm_bindgen(method, getter = rowSpan, structural)]
    pub fn row_span(this: &HTMLTableCellElement) -> u32;

    #[wasm_bindgen(method, setter = rowSpan, structural)]
    pub fn set_row_span(this: &HTMLTableCellElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn scope(this: &HTMLTableCellElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_scope(this: &HTMLTableCellElement, val: String);

    #[wasm_bindgen(method, getter = cellIndex, structural)]
    pub fn cell_index(this: &HTMLTableCellElement) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn span(this: &HTMLTableColElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_span(this: &HTMLTableColElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rows(this: &HTMLTableElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter = tBodies, structural)]
    pub fn t_bodies(this: &HTMLTableElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter, structural)]
    pub fn cells(this: &HTMLTableRowElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter = rowIndex, structural)]
    pub fn row_index(this: &HTMLTableRowElement) -> i32;

    #[wasm_bindgen(method, getter = sectionRowIndex, structural)]
    pub fn section_row_index(this: &HTMLTableRowElement) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn rows(this: &HTMLTableSectionElement) -> HTMLCollection;

    #[wasm_bindgen(method, getter, structural)]
    pub fn content(this: &HTMLTemplateElement) -> DocumentFragment;

    #[wasm_bindgen(method, getter, structural)]
    pub fn autocomplete(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autocomplete(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn autofocus(this: &HTMLTextAreaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_autofocus(this: &HTMLTextAreaElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn cols(this: &HTMLTextAreaElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_cols(this: &HTMLTextAreaElement, val: u32);

    #[wasm_bindgen(method, getter = defaultValue, structural)]
    pub fn default_value(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter = defaultValue, structural)]
    pub fn set_default_value(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter = dirName, structural)]
    pub fn dir_name(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter = dirName, structural)]
    pub fn set_dir_name(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn disabled(this: &HTMLTextAreaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_disabled(this: &HTMLTextAreaElement, val: bool);

    #[wasm_bindgen(method, getter = maxLength, structural)]
    pub fn max_length(this: &HTMLTextAreaElement) -> i32;

    #[wasm_bindgen(method, setter = maxLength, structural)]
    pub fn set_max_length(this: &HTMLTextAreaElement, val: i32);

    #[wasm_bindgen(method, getter = minLength, structural)]
    pub fn min_length(this: &HTMLTextAreaElement) -> i32;

    #[wasm_bindgen(method, setter = minLength, structural)]
    pub fn set_min_length(this: &HTMLTextAreaElement, val: i32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_name(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn placeholder(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_placeholder(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter = readOnly, structural)]
    pub fn read_only(this: &HTMLTextAreaElement) -> bool;

    #[wasm_bindgen(method, setter = readOnly, structural)]
    pub fn set_read_only(this: &HTMLTextAreaElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn required(this: &HTMLTextAreaElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_required(this: &HTMLTextAreaElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn rows(this: &HTMLTextAreaElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_rows(this: &HTMLTextAreaElement, val: u32);

    #[wasm_bindgen(method, getter = selectionDirection, structural)]
    pub fn selection_direction(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter = selectionDirection, structural)]
    pub fn set_selection_direction(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter = selectionEnd, structural)]
    pub fn selection_end(this: &HTMLTextAreaElement) -> u32;

    #[wasm_bindgen(method, setter = selectionEnd, structural)]
    pub fn set_selection_end(this: &HTMLTextAreaElement, val: u32);

    #[wasm_bindgen(method, getter = selectionStart, structural)]
    pub fn selection_start(this: &HTMLTextAreaElement) -> u32;

    #[wasm_bindgen(method, setter = selectionStart, structural)]
    pub fn set_selection_start(this: &HTMLTextAreaElement, val: u32);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn wrap(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_wrap(this: &HTMLTextAreaElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn labels(this: &HTMLTextAreaElement) -> NodeList;

    #[wasm_bindgen(method, getter = textLength, structural)]
    pub fn text_length(this: &HTMLTextAreaElement) -> u32;

    #[wasm_bindgen(method, getter = validationMessage, structural)]
    pub fn validation_message(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn validity(this: &HTMLTextAreaElement) -> ValidityState;

    #[wasm_bindgen(method, getter = willValidate, structural)]
    pub fn will_validate(this: &HTMLTextAreaElement) -> bool;

    #[wasm_bindgen(method, getter = dateTime, structural)]
    pub fn date_time(this: &HTMLTimeElement) -> String;

    #[wasm_bindgen(method, setter = dateTime, structural)]
    pub fn set_date_time(this: &HTMLTimeElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn text(this: &HTMLTitleElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_text(this: &HTMLTitleElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn default(this: &HTMLTrackElement) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_default(this: &HTMLTrackElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn kind(this: &HTMLTrackElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_kind(this: &HTMLTrackElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &HTMLTrackElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_label(this: &HTMLTrackElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn src(this: &HTMLTrackElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_src(this: &HTMLTrackElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn srclang(this: &HTMLTrackElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_srclang(this: &HTMLTrackElement, val: String);

    #[wasm_bindgen(method, getter = readyState, structural)]
    pub fn ready_state(this: &HTMLTrackElement) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn track(this: &HTMLTrackElement) -> TextTrack;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLVideoElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &HTMLVideoElement, val: u32);

    #[wasm_bindgen(method, getter = playsInline, structural)]
    pub fn plays_inline(this: &HTMLVideoElement) -> bool;

    #[wasm_bindgen(method, setter = playsInline, structural)]
    pub fn set_plays_inline(this: &HTMLVideoElement, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn poster(this: &HTMLVideoElement) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_poster(this: &HTMLVideoElement, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLVideoElement) -> u32;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &HTMLVideoElement, val: u32);

    #[wasm_bindgen(method, getter = videoHeight, structural)]
    pub fn video_height(this: &HTMLVideoElement) -> u32;

    #[wasm_bindgen(method, getter = videoWidth, structural)]
    pub fn video_width(this: &HTMLVideoElement) -> u32;

    #[wasm_bindgen(method, getter = newURL, structural)]
    pub fn new_url(this: &HashChangeEvent) -> String;

    #[wasm_bindgen(method, getter = oldURL, structural)]
    pub fn old_url(this: &HashChangeEvent) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &History) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &ImageBitmap) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &ImageBitmap) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn canvas(this: &ImageBitmapRenderingContext) -> HTMLCanvasElement;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &ImageData) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &ImageData) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn hash(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hash(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn host(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_host(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn hostname(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_hostname(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn href(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_href(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn pathname(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_pathname(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn port(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_port(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn protocol(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_protocol(this: &Location, val: String);

    #[wasm_bindgen(method, getter, structural)]
    pub fn search(this: &Location) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_search(this: &Location, val: String);

    #[wasm_bindgen(method, getter = ancestorOrigins, structural)]
    pub fn ancestor_origins(this: &Location) -> DOMStringList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &Location) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn code(this: &MediaError) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn message(this: &MediaError) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn port1(this: &MessageChannel) -> MessagePort;

    #[wasm_bindgen(method, getter, structural)]
    pub fn port2(this: &MessageChannel) -> MessagePort;

    #[wasm_bindgen(method, getter = lastEventId, structural)]
    pub fn last_event_id(this: &MessageEvent) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &MessageEvent) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn description(this: &MimeType) -> String;

    #[wasm_bindgen(method, getter = enabledPlugin, structural)]
    pub fn enabled_plugin(this: &MimeType) -> Plugin;

    #[wasm_bindgen(method, getter, structural)]
    pub fn suffixes(this: &MimeType) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &MimeTypeArray) -> u32;

    #[wasm_bindgen(method, getter = addedNodes, structural)]
    pub fn added_nodes(this: &MutationRecord) -> NodeList;

    #[wasm_bindgen(method, getter = attributeName, structural)]
    pub fn attribute_name(this: &MutationRecord) -> String;

    #[wasm_bindgen(method, getter = attributeNamespace, structural)]
    pub fn attribute_namespace(this: &MutationRecord) -> String;

    #[wasm_bindgen(method, getter = oldValue, structural)]
    pub fn old_value(this: &MutationRecord) -> String;

    #[wasm_bindgen(method, getter = removedNodes, structural)]
    pub fn removed_nodes(this: &MutationRecord) -> NodeList;

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &MutationRecord) -> Node;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &NamedNodeMap) -> u32;

    #[wasm_bindgen(method, getter = nodeValue, structural)]
    pub fn node_value(this: &Node) -> String;

    #[wasm_bindgen(method, setter = nodeValue, structural)]
    pub fn set_node_value(this: &Node, val: String);

    #[wasm_bindgen(method, getter = textContent, structural)]
    pub fn text_content(this: &Node) -> String;

    #[wasm_bindgen(method, setter = textContent, structural)]
    pub fn set_text_content(this: &Node, val: String);

    #[wasm_bindgen(method, getter = baseURI, structural)]
    pub fn base_uri(this: &Node) -> String;

    #[wasm_bindgen(method, getter = childNodes, structural)]
    pub fn child_nodes(this: &Node) -> NodeList;

    #[wasm_bindgen(method, getter = isConnected, structural)]
    pub fn is_connected(this: &Node) -> bool;

    #[wasm_bindgen(method, getter = nodeName, structural)]
    pub fn node_name(this: &Node) -> String;

    #[wasm_bindgen(method, getter = nodeType, structural)]
    pub fn node_type(this: &Node) -> u16;

    #[wasm_bindgen(method, getter = pointerBeforeReferenceNode, structural)]
    pub fn pointer_before_reference_node(this: &NodeIterator) -> bool;

    #[wasm_bindgen(method, getter = referenceNode, structural)]
    pub fn reference_node(this: &NodeIterator) -> Node;

    #[wasm_bindgen(method, getter, structural)]
    pub fn root(this: &NodeIterator) -> Node;

    #[wasm_bindgen(method, getter = whatToShow, structural)]
    pub fn what_to_show(this: &NodeIterator) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &NodeList) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &OffscreenCanvas) -> u64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_height(this: &OffscreenCanvas, val: u64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &OffscreenCanvas) -> u64;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_width(this: &OffscreenCanvas, val: u64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn canvas(this: &OffscreenCanvasRenderingContext2D) -> OffscreenCanvas;

    #[wasm_bindgen(method, getter, structural)]
    pub fn persisted(this: &PageTransitionEvent) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn description(this: &Plugin) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn filename(this: &Plugin) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Plugin) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn name(this: &Plugin) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &PluginArray) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn target(this: &ProcessingInstruction) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &RadioNodeList) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_value(this: &RadioNodeList, val: String);

    #[wasm_bindgen(method, getter = commonAncestorContainer, structural)]
    pub fn common_ancestor_container(this: &Range) -> Node;

    #[wasm_bindgen(method, getter, structural)]
    pub fn host(this: &ShadowRoot) -> Element;

    #[wasm_bindgen(method, getter, structural)]
    pub fn port(this: &SharedWorker) -> MessagePort;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &Storage) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn key(this: &StorageEvent) -> String;

    #[wasm_bindgen(method, getter = newValue, structural)]
    pub fn new_value(this: &StorageEvent) -> String;

    #[wasm_bindgen(method, getter = oldValue, structural)]
    pub fn old_value(this: &StorageEvent) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn url(this: &StorageEvent) -> String;

    #[wasm_bindgen(method, getter = wholeText, structural)]
    pub fn whole_text(this: &Text) -> String;

    #[wasm_bindgen(method, getter = actualBoundingBoxAscent, structural)]
    pub fn actual_bounding_box_ascent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = actualBoundingBoxDescent, structural)]
    pub fn actual_bounding_box_descent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = actualBoundingBoxLeft, structural)]
    pub fn actual_bounding_box_left(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = actualBoundingBoxRight, structural)]
    pub fn actual_bounding_box_right(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = alphabeticBaseline, structural)]
    pub fn alphabetic_baseline(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = emHeightAscent, structural)]
    pub fn em_height_ascent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = emHeightDescent, structural)]
    pub fn em_height_descent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = fontBoundingBoxAscent, structural)]
    pub fn font_bounding_box_ascent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = fontBoundingBoxDescent, structural)]
    pub fn font_bounding_box_descent(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = hangingBaseline, structural)]
    pub fn hanging_baseline(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter = ideographicBaseline, structural)]
    pub fn ideographic_baseline(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &TextMetrics) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &TextTrack) -> String;

    #[wasm_bindgen(method, getter = inBandMetadataTrackDispatchType, structural)]
    pub fn in_band_metadata_track_dispatch_type(this: &TextTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &TextTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn language(this: &TextTrack) -> String;

    #[wasm_bindgen(method, getter = endTime, structural)]
    pub fn end_time(this: &TextTrackCue) -> f64;

    #[wasm_bindgen(method, setter = endTime, structural)]
    pub fn set_end_time(this: &TextTrackCue, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &TextTrackCue) -> String;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_id(this: &TextTrackCue, val: String);

    #[wasm_bindgen(method, getter = pauseOnExit, structural)]
    pub fn pause_on_exit(this: &TextTrackCue) -> bool;

    #[wasm_bindgen(method, setter = pauseOnExit, structural)]
    pub fn set_pause_on_exit(this: &TextTrackCue, val: bool);

    #[wasm_bindgen(method, getter = startTime, structural)]
    pub fn start_time(this: &TextTrackCue) -> f64;

    #[wasm_bindgen(method, setter = startTime, structural)]
    pub fn set_start_time(this: &TextTrackCue, val: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &TextTrackCueList) -> u32;

    #[wasm_bindgen(method, getter = currentNode, structural)]
    pub fn current_node(this: &TreeWalker) -> Node;

    #[wasm_bindgen(method, setter = currentNode, structural)]
    pub fn set_current_node(this: &TreeWalker, val: Node);

    #[wasm_bindgen(method, getter, structural)]
    pub fn root(this: &TreeWalker) -> Node;

    #[wasm_bindgen(method, getter = whatToShow, structural)]
    pub fn what_to_show(this: &TreeWalker) -> u32;

    #[wasm_bindgen(method, getter = badInput, structural)]
    pub fn bad_input(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = customError, structural)]
    pub fn custom_error(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = patternMismatch, structural)]
    pub fn pattern_mismatch(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = rangeOverflow, structural)]
    pub fn range_overflow(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = rangeUnderflow, structural)]
    pub fn range_underflow(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = stepMismatch, structural)]
    pub fn step_mismatch(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = tooLong, structural)]
    pub fn too_long(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = tooShort, structural)]
    pub fn too_short(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = typeMismatch, structural)]
    pub fn type_mismatch(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn valid(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter = valueMissing, structural)]
    pub fn value_missing(this: &ValidityState) -> bool;

    #[wasm_bindgen(method, getter, structural)]
    pub fn selected(this: &VideoTrack) -> bool;

    #[wasm_bindgen(method, setter, structural)]
    pub fn set_selected(this: &VideoTrack, val: bool);

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &VideoTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn kind(this: &VideoTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn label(this: &VideoTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn language(this: &VideoTrack) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn length(this: &VideoTrackList) -> u32;

    #[wasm_bindgen(method, getter = selectedIndex, structural)]
    pub fn selected_index(this: &VideoTrackList) -> i32;

    #[wasm_bindgen(method, getter = bufferedAmount, structural)]
    pub fn buffered_amount(this: &WebSocket) -> u64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn extensions(this: &WebSocket) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn protocol(this: &WebSocket) -> String;

    #[wasm_bindgen(method, getter = readyState, structural)]
    pub fn ready_state(this: &WebSocket) -> u16;

    #[wasm_bindgen(method, getter, structural)]
    pub fn url(this: &WebSocket) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn location(this: &WorkerGlobalScope) -> WorkerLocation;

    #[wasm_bindgen(method, getter, structural)]
    pub fn navigator(this: &WorkerGlobalScope) -> WorkerNavigator;

    #[wasm_bindgen(method, getter, structural)]
    pub fn hash(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn host(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn hostname(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn href(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn pathname(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn port(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn protocol(this: &WorkerLocation) -> String;

    #[wasm_bindgen(method, getter, structural)]
    pub fn search(this: &WorkerLocation) -> String;

    pub static name: String;

    pub static status: String;

    #[wasm_bindgen(js_name = applicationCache)]
    pub static application_cache: ApplicationCache;

    pub static closed: bool;

    #[wasm_bindgen(js_name = customElements)]
    pub static custom_elements: CustomElementRegistry;

    pub static document: Document;

    pub static history: History;

    pub static length: u32;

    pub static location: Location;

    pub static locationbar: BarProp;

    pub static menubar: BarProp;

    pub static navigator: Navigator;

    pub static personalbar: BarProp;

    pub static scrollbars: BarProp;

    pub static statusbar: BarProp;

    pub static toolbar: BarProp;
}
        