#![allow(non_upper_case_globals)]

type Attribute = ();

#[cfg(feature = "well-known-alpinejs")]
#[rustfmt::skip]
pub trait AlpineJsAttributes {
    /// `x-data` defines a chunk of HTML as an Alpine component and provides the reactive data for that component to reference.
    const x_data: Attribute = ();
    /// The `x-init` directive allows you to hook into the initialization phase of any element in Alpine.
    const x_init: Attribute = ();
    /// `x-show` is one of the most useful and powerful directives in Alpine. It provides an expressive way to show and hide DOM elements.
    const x_show: Attribute = ();
    /// `x-text` sets the text content of an element to the result of a given expression.
    const x_text: Attribute = ();
    /// `x-html` sets the "innerHTML" property of an element to the result of a given expression.
    const x_html: Attribute = ();
    /// `x-model` allows you to bind the value of an input element to Alpine data.
    const x_model: Attribute = ();
    /// `x-modelable` allows you to expose any Alpine property as the target of the `x-model` directive.
    const x_modelable: Attribute = ();
    /// Alpine's `x-for` directive allows you to create DOM elements by iterating through a list.
    const x_for: Attribute = ();
    /// Alpine provides a robust transitions utility out of the box. With a few `x-transition` directives, you can create smooth transitions between when an element is shown or hidden.
    const x_transition: Attribute = ();
    /// `x-effect` is a useful directive for re-evaluating an expression when one of its dependencies change. You can think of it as a watcher where you don't have to specify what property to watch, it will watch all properties used within it.
    const x_effect: Attribute = ();
    /// By default, Alpine will crawl and initialize the entire DOM tree of an element containing `x-init` or `x-data`. If for some reason, you don't want Alpine to touch a specific section of your HTML, you can prevent it from doing so using `x-ignore`.
    const x_ignore: Attribute = ();
    /// `x-ref` in combination with `$refs` is a useful utility for easily accessing DOM elements directly. It's most useful as a replacement for APIs like `getElementById` and `querySelector`.
    const x_ref: Attribute = ();
    /// Sometimes, when you're using AlpineJS for a part of your template, there is a "blip" where you might see your uninitialized template after the page loads, but before Alpine loads. `x-cloak` addresses this scenario by hiding the element it's attached to until Alpine is fully loaded on the page.
    const x_cloak: Attribute = ();
    /// The `x-teleport` directive allows you to transport part of your Alpine template to another part of the DOM on the page entirely.
    const x_teleport: Attribute = ();
    /// `x-if` is used for toggling elements on the page, similarly to `x-show`, however it completely adds and removes the element it's applied to rather than just changing its CSS display property to "none".
    const x_if: Attribute = ();
    /// `x-id` allows you to declare a new "scope" for any new IDs generated using `$idAttribute`. It accepts an array of strings (ID names) and adds a suffix to each `$id('...')` generated within it that is unique to other IDs on the page. `x-id` is meant to be used in conjunction with the `$id(...)` magic.
    const x_id: Attribute = ();
}

#[cfg(feature = "well-known-alpinejs")]
impl<T: html5::Element> AlpineJsAttributes for T {}

#[cfg(feature = "well-known-htmx")]
#[rustfmt::skip]
pub trait HtmxAttributes {
    /// issues a `GET` to the specified URL
    const hx_get: Attribute = ();
    /// issues a `POST` to the specified URL
    const hx_post: Attribute = ();
    /// push a URL into the browser location bar to create history
    const hx_push_url: Attribute = ();
    /// select content to swap in from a response
    const hx_select: Attribute = ();
    /// select content to swap in from a response, somewhere other than the target (out of band)
    const hx_select_oob: Attribute = ();
    /// controls how content will swap in (`outerHTML`, `beforeend`, `afterend`, …)
    const hx_swap: Attribute = ();
    /// mark element to swap in from a response (out of band)
    const hx_swap_oob: Attribute = ();
    /// specifies the target element to be swapped
    const hx_target: Attribute = ();
    /// specifies the event that triggers the request
    const hx_trigger: Attribute = ();
    /// add values to submit with the request (JSON format)
    const hx_vals: Attribute = ();
    /// add progressive enhancement for links and forms
    const hx_boost: Attribute = ();
    /// shows a `confirmAttribute` dialog before issuing a request
    const hx_confirm: Attribute = ();
    /// issues a `DELETE` to the specified URL
    const hx_delete: Attribute = ();
    /// disables htmx processing for the given node and any children nodes
    const hx_disable: Attribute = ();
    /// adds the `disabled` attribute to the specified elements while a request is in flight
    const hx_disabled_elt: Attribute = ();
    /// control and disable automatic attribute inheritance for child nodes
    const hx_disinherit: Attribute = ();
    /// changes the request encoding type
    const hx_encoding: Attribute = ();
    /// extensions to use for this element
    const hx_ext: Attribute = ();
    /// adds to the headers that will be submitted with the request
    const hx_headers: Attribute = ();
    /// prevent sensitive data being saved to the history cache
    const hx_history: Attribute = ();
    /// the element to snapshot and restore during history navigation
    const hx_history_elt: Attribute = ();
    /// include additional data in requests
    const hx_include: Attribute = ();
    /// the element to put the `htmx-request` class on during the request
    const hx_indicator: Attribute = ();
    /// control and enable automatic attribute inheritance for child nodes if it has been disabled by default
    const hx_inherit: Attribute = ();
    /// filters the parameters that will be submitted with a request
    const hx_params: Attribute = ();
    /// issues a `PATCH` to the specified URL
    const hx_patch: Attribute = ();
    /// specifies elements to keep unchanged between requests
    const hx_preserve: Attribute = ();
    /// shows a `promptAttribute` before submitting a request
    const hx_prompt: Attribute = ();
    /// issues a `PUT` to the specified URL
    const hx_put: Attribute = ();
    /// replace the URL in the browser location bar
    const hx_replace_url: Attribute = ();
    /// configures various aspects of the request
    const hx_request: Attribute = ();
    /// control how requests made by different elements are synchronized
    const hx_sync: Attribute = ();
    /// force elements to validate themselves before a request
    const hx_validate: Attribute = ();
    /// adds values dynamically to the parameters to submit with the request (deprecated, please use `hx-vals`)
    const hx_vars: Attribute = ();
}

#[cfg(feature = "well-known-htmx")]
impl<T: html5::Element> HtmxAttributes for T {}

#[cfg(feature = "well-known-html5")]
#[macro_use]
pub mod html5 {
    use vy_macros::_define_html5_elements;

    use super::Attribute;

    pub trait Element {}

    #[rustfmt::skip]
    pub trait GlobalAttributes {
        /// Provides a hint for generating a keyboard shortcut for the current element. This attribute consists of a space-separated list of characters. The browser should use the first one that exists on the computer keyboard layout. 
        const accesskey: Attribute = ();
        /// Controls whether inputted text is automatically capitalized and, if so, in what manner. 
        const autocapitalize: Attribute = ();
        /// Controls whether input text is automatically corrected for spelling errors. This can be applied to elements that have editable text except for <input> elements with the attribute: type="password", type="email", or type="url". 
        const autocorrect: Attribute = ();
        /// Indicates that an element is to be focused on page load, or as soon as the <dialog> it is part of is displayed. This attribute is a boolean, initially false. 
        const autofocus: Attribute = ();
        /// A space-separated list of the classes of the element. Classes allow CSS and JavaScript to select and access specific elements via the class selectors or functions like the method Document.getElementsByClassNameAttribute. 
        const class: Attribute = ();
        /// An enumerated attribute indicating if the element should be editable by the user. If so, the browser modifies its widget to allow editing. The attribute must take one of the following values: true or the empty string, which indicates that the element must be editable; false, which indicates that the element must not be editable. plaintext-only, which indicates the element's raw text is editable, but rich text formatting is disabled. 
        const contenteditable: Attribute = ();
        /// An enumerated attribute indicating the directionality of the element's text. It can have the following values: ltr, which means left to right and is to be used for languages that are written from the left to the right (like English); rtl, which means right to left and is to be used for languages that are written from the right to the left (like Arabic); auto, which lets the user agent decide. It uses a basic algorithm as it parses the characters inside the element until it finds a character with a strong directionality, then it applies that directionality to the whole element. 
        const dir: Attribute = ();
        /// An enumerated attribute indicating whether the element can be dragged, using the Drag and Drop API. It can have the following values: true, which indicates that the element may be dragged false, which indicates that the element may not be dragged. 
        const draggable: Attribute = ();
        /// Hints what action label (or icon) to present for the enter key on virtual keyboards. 
        const enterkeyhint: Attribute = ();
        /// Used to transitively export shadow parts from a nested shadow tree into a containing light tree. 
        const exportparts: Attribute = ();
        /// An enumerated attribute indicating that the element is not yet, or is no longer, relevant. For example, it can be used to hide elements of the page that can't be used until the login process has been completed. The browser won't render such elements. This attribute must not be used to hide content that could legitimately be shown. 
        const hidden: Attribute = ();
        /// Defines a unique identifier (ID) which must be unique in the whole document. Its purpose is to identify the element when linking (using a fragment identifier), scripting, or styling (with CSS). 
        const id: Attribute = ();
        /// A boolean value that makes the browser disregard user input events for the element. Useful when click events are present. 
        const inert: Attribute = ();
        /// Provides a hint to browsers about the type of virtual keyboard configuration to use when editing this element or its contents. Used primarily on <input> elements, but is usable on any element while in contenteditable mode. 
        const inputmode: Attribute = ();
        /// Allows you to specify that a standard HTML element should behave like a registered customized built-in element (see Using custom elements for more details). 
        const is: Attribute = ();
        /// The unique, global identifier of an item. 
        const itemid: Attribute = ();
        /// Used to add properties to an item. Every HTML element may have an itemprop attribute specified, where an itemprop consists of a name and value pair. 
        const itemprop: Attribute = ();
        /// Properties that are not descendants of an element with the itemscope attribute can be associated with the item using an itemref. It provides a list of element ids (not itemids) with additional properties elsewhere in the document. 
        const itemref: Attribute = ();
        /// itemscope (usually) works along with itemtype to specify that the HTML contained in a block is about a particular item. itemscope creates the Item and defines the scope of the itemtype associated with it. itemtype is a valid URL of a vocabulary (such as schema.org) that describes the item and its properties context. 
        const itemscope: Attribute = ();
        /// Specifies the URL of the vocabulary that will be used to define itemprops (item properties) in the data structure. itemscope is used to set the scope of where in the data structure the vocabulary set by itemtype will be active. 
        const itemtype: Attribute = ();
        /// Helps define the language of an element: the language that non-editable elements are in, or the language that editable elements should be written in by the user. The attribute should contain a valid BCP 47 language tag. xml:lang has priority over it. 
        const lang: Attribute = ();
        /// A cryptographic nonce ("number used once") which can be used by Content Security Policy to determine whether or not a given fetch will be allowed to proceed. 
        const nonce: Attribute = ();
        /// A space-separated list of the part names of the element. Part names allows CSS to select and style specific elements in a shadow tree via the ::part pseudo-element. 
        const part: Attribute = ();
        /// Used to designate an element as a popover element (see Popover API). Popover elements are hidden via display: none until opened via an invoking/control element (i.e., a <button> or <input type="button"> with a popovertarget attribute) or a HTMLElement.showPopoverAttribute call. 
        const popover: Attribute = ();
        /// Roles define the semantic meaning of content, allowing screen readers and other tools to present and support interaction with an object in a way that is consistent with user expectations of that type of object. roles are added to HTML elements using role="role_type", where role_type is the name of a role in the ARIA specification. 
        const role: Attribute = ();
        /// Assigns a slot in a shadow DOM shadow tree to an element: An element with a slot attribute is assigned to the slot created by the <slot> element whose name attribute's value matches that slot attribute's value. 
        const slot: Attribute = ();
        /// An enumerated attribute defines whether the element may be checked for spelling errors. It may have the following values: empty string or true, which indicates that the element should be, if possible, checked for spelling errors; false, which indicates that the element should not be checked for spelling errors. 
        const spellcheck: Attribute = ();
        /// Contains CSS styling declarations to be applied to the element. Note that it is recommended for styles to be defined in a separate file or files. This attribute and the <style> element have mainly the purpose of allowing for quick styling, for example for testing purposes. 
        const style: Attribute = ();
        /// An integer attribute indicating if the element can take input focus (is focusable), if it should participate to sequential keyboard navigation, and if so, at what position. It can take several values: a negative value means that the element should be focusable, but should not be reachable via sequential keyboard navigation; 0 means that the element should be focusable and reachable via sequential keyboard navigation, but its relative order is defined by the platform convention; a positive value means that the element should be focusable and reachable via sequential keyboard navigation; the order in which the elements are focused is the increasing value of the tabindex. If several elements share the same tabindex, their relative order follows their relative positions in the document. 
        const tabindex: Attribute = ();
        /// Contains a text representing advisory information related to the element it belongs to. Such information can typically, but not necessarily, be presented to the user as a tooltip. 
        const title: Attribute = ();
        /// An enumerated attribute that is used to specify whether an element's attribute values and the values of its Text node children are to be translated when the page is localized, or whether to leave them unchanged. It can have the following values: empty string or yes, which indicates that the element will be translated. no, which indicates that the element will not be translated. 
        const translate: Attribute = ();
        /// An enumerated attribute indicating if browser-provided writing suggestions should be enabled under the scope of the element or not. false, which disables the browser's writing suggestions. true or an empty string, which enables writing suggestions. 
        const writingsuggestions: Attribute = ();
    }

    impl<T: Element> GlobalAttributes for T {}

    pub trait AriaAttributes {}

    impl<T: Element> AriaAttributes for T {}

    _define_html5_elements! {
        /// Together with its href attribute, creates a hyperlink to web pages, files, email addresses, locations within the current page, or anything else a URL can address.
        a impl A {
            pub const download: Attribute = ();
            pub const href: Attribute = ();
            pub const hreflang: Attribute = ();
            pub const interestfor: Attribute = ();
            pub const ping: Attribute = ();
            pub const referrerpolicy: Attribute = ();
            pub const target: Attribute = ();
            pub const r#type: Attribute = ();
        },
        /// Represents an abbreviation or acronym.
        abbr impl Abbr {},
        /// Indicates that the enclosed HTML provides contact information for a person or people, or for an organization.
        address impl Address {},
        /// Defines an area inside an image map that has predefined clickable areas. An image map allows geometric areas on an image to be associated with hyperlink.
        area impl Area {
            pub const alt: Attribute = ();
            pub const coords: Attribute = ();
            pub const href: Attribute = ();
            pub const media: Attribute = ();
            pub const shape: Attribute = ();
            pub const target: Attribute = ();
            pub const download: Attribute = ();
        },
        /// Represents a self-contained composition in a document, page, application, or site, which is intended to be independently distributable or reusable (e.g., in syndication). Examples include a forum post, a magazine or newspaper article, a blog entry, a product card, a user-submitted comment, an interactive widget or gadget, or any other independent item of content.
        article impl Article {},
        /// Represents a portion of a document whose content is only indirectly related to the document's main content. Asides are frequently presented as sidebars or call-out boxes.
        aside impl Aside {},
        /// Used to embed sound content in documents. It may contain one or more audio sources, represented using the src attribute or the source element: the browser will choose the most suitable one. It can also be the destination for streamed media, using a MediaStream.
        audio impl Audio {
            pub const autoplay: Attribute = ();
            pub const controls: Attribute = ();
            pub const r#loop: Attribute = ();
            pub const muted: Attribute = ();
            pub const preload: Attribute = ();
            pub const src: Attribute = ();
        },
        /// Used to draw the reader's attention to the element's contents, which are not otherwise granted special importance. This was formerly known as the Boldface element, and most browsers still draw the text in boldface. However, you should not use <b> for styling text or granting importance. If you wish to create boldface text, you should use the CSS font-weight property. If you wish to indicate an element is of special importance, you should use the <strong> element.
        b impl B {},
        /// Specifies the base URL to use for all relative URLs in a document. There can be only one such element in a document.
        base impl Base {
            pub const href: Attribute = ();
        },
        /// Tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text. It's particularly useful when a website dynamically inserts some text and doesn't know the directionality of the text being inserted.
        bdi impl Bdi {},
        /// Overrides the current directionality of text, so that the text within is rendered in a different direction.
        bdo impl Bdo {
            pub const dir: Attribute = ();
        },
        /// Indicates that the enclosed text is an extended quotation. Usually, this is rendered visually by indentation. A URL for the source of the quotation may be given using the cite attribute, while a text representation of the source can be given using the <cite> element.
        blockquote impl Blockquote {
            pub const cite: Attribute = ();
        },
        /// Represents the content of an HTML document. There can be only one such element in a document.
        body impl Body {},
        /// Produces a line break in text (carriage-return). It is useful for writing a poem or an address, where the division of lines is significant.
        br impl Br {},
        /// An interactive element activated by a user with a mouse, keyboard, finger, voice command, or other assistive technology. Once activated, it performs an action, such as submitting a form or opening a dialog.
        button impl Button {
            pub const autofocus: Attribute = ();
            pub const disabled: Attribute = ();
            pub const form: Attribute = ();
            pub const formaction: Attribute = ();
            pub const formenctype: Attribute = ();
            pub const formmethod: Attribute = ();
            pub const formnovalidate: Attribute = ();
            pub const formtarget: Attribute = ();
            pub const name: Attribute = ();
            pub const r#type: Attribute = ();
            pub const value: Attribute = ();
        },
        /// Container element to use with either the canvas scripting API or the WebGL API to draw graphics and animations.
        canvas impl Canvas {
            pub const width: Attribute = ();
            pub const height: Attribute = ();
        },
        /// Specifies the caption (or title) of a table.
        caption impl Caption {},
        /// Used to mark up the title of a creative work. The reference may be in an abbreviated form according to context-appropriate conventions related to citation metadata.
        cite impl Cite {},
        /// Displays its contents styled in a fashion intended to indicate that the text is a short fragment of computer code. By default, the content text is displayed using the user agent's default monospace font.
        code impl Code {},
        /// Defines one or more columns in a column group represented by its implicit or explicit parent <colgroup> element. The <col> element is only valid as a child of a <colgroup> element that has no span attribute defined.
        col impl Col {
            pub const span: Attribute = ();
            pub const width: Attribute = ();
        },
        /// Defines a group of columns within a table.
        colgroup impl Colgroup {
            pub const span: Attribute = ();
        },
        /// Links a given piece of content with a machine-readable translation. If the content is time- or date-related, the <time> element must be used.
        data impl Data {
            pub const value: Attribute = ();
        },
        /// Contains a set of <option> elements that represent the permissible or recommended options available to choose from within other controls.
        datalist impl Datalist {},
        /// Provides the description, definition, or value for the preceding term (<dt>) in a description list (<dl>).
        dd impl Dd {},
        /// Represents a range of text that has been deleted from a document. This can be used when rendering "track changes" or source code diff information, for example. The <ins> element can be used for the opposite purpose: to indicate text that has been added to the document.
        del impl Del {
            pub const cite: Attribute = ();
            pub const datetime: Attribute = ();
        },
        /// Creates a disclosure widget in which information is visible only when the widget is toggled into an "open" state. A summary or label must be provided using the <summary> element.
        details impl Details {
            pub const open: Attribute = ();
        },
        /// Used to indicate the term being defined within the context of a definition phrase or sentence. The ancestor <p> element, the <dt>/<dd> pairing, or the nearest section ancestor of the <dfn> element, is considered to be the definition of the term.
        dfn impl Dfn {},
        /// Represents a dialog box or other interactive component, such as a dismissible alert, inspector, or subwindow.
        dialog impl Dialog {
            pub const open: Attribute = ();
        },
        /// The generic container for flow content. It has no effect on the content or layout until styled in some way using CSS (e.g., styling is directly applied to it, or some kind of layout model like flexbox is applied to its parent element).
        div impl Div {},
        /// Represents a description list. The element encloses a list of groups of terms (specified using the <dt> element) and descriptions (provided by <dd> elements). Common uses for this element are to implement a glossary or to display metadata (a list of key-value pairs).
        dl impl Dl {},
        /// Specifies a term in a description or definition list, and as such must be used inside a <dl> element. It is usually followed by a <dd> element; however, multiple <dt> elements in a row indicate several terms that are all defined by the immediate next <dd> element.
        dt impl Dt {},
        /// Marks text that has stress emphasis. The <em> element can be nested, with each nesting level indicating a greater degree of emphasis.
        em impl Em {},
        /// Embeds external content at the specified point in the document. This content is provided by an external application or other source of interactive content such as a browser plug-in.
        embed impl Embed {
            pub const r#type: Attribute = ();
            pub const src: Attribute = ();
        },
        /// Represents a nested browsing context, like <iframe> but with more native privacy features built in.
        fencedframe impl Fencedframe {},
        /// Used to group several controls as well as labels (<label>) within a web form.
        fieldset impl Fieldset {
            pub const disabled: Attribute = ();
            pub const form: Attribute = ();
            pub const name: Attribute = ();
        },
        /// Represents a caption or legend describing the rest of the contents of its parent <figure> element.
        figcaption impl Figcaption {},
        /// Represents self-contained content, potentially with an optional caption, which is specified using the <figcaption> element. The figure, its caption, and its contents are referenced as a single unit.
        figure impl Figure {},
        /// Represents a footer for its nearest ancestor sectioning content or sectioning root element. A <footer> typically contains information about the author of the section, copyright data, or links to related documents.
        footer impl Footer {},
        /// Represents a document section containing interactive controls for submitting information.
        form impl Form {
            pub const action: Attribute = ();
            pub const method: Attribute = ();
            pub const enctype: Attribute = ();
            pub const target: Attribute = ();
            pub const name: Attribute = ();
        },
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h1 impl H1 {},
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h2 impl H2 {},
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h3 impl H3 {},
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h4 impl H4 {},
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h5 impl H5 {},
        /// Represent six levels of section headings. <h1> is the highest section level and <h6> is the lowest.
        h6 impl H6 {},
        /// Contains machine-readable information (metadata) about the document, like its title, scripts, and style sheets.
        head impl Head {},
        /// Represents introductory content, typically a group of introductory or navigational aids. It may contain some heading elements but also a logo, a search form, an author name, and other elements.
        header impl Header {},
        /// Represents a heading grouped with any secondary content, such as subheadings, an alternative title, or a tagline.
        hgroup impl Hgroup {},
        /// Represents a thematic break between paragraph-level elements: for example, a change of scene in a story, or a shift of topic within a section.
        hr impl Hr {},
        /// Represents the root (top-level element) of an HTML document, so it is also referred to as the root element. All other elements must be descendants of this element.
        html impl Html {},
        /// Represents a range of text that is set off from the normal text for some reason, such as idiomatic text, technical terms, and taxonomical designations, among others. Historically, these have been presented using italicized type, which is the original source of the <i> naming of this element.
        i impl I {},
        /// Represents a nested browsing context, embedding another HTML page into the current one.
        iframe impl Iframe {
            pub const src: Attribute = ();
            pub const width: Attribute = ();
            pub const height: Attribute = ();
            pub const name: Attribute = ();
            pub const sandbox: Attribute = ();
            pub const frameborder: Attribute = ();
            pub const scrolling: Attribute = ();
        },
        /// Embeds an image into the document.
        img impl Img {
            pub const src: Attribute = ();
            pub const alt: Attribute = ();
            pub const width: Attribute = ();
            pub const height: Attribute = ();
            pub const srcset: Attribute = ();
            pub const sizes: Attribute = ();
        },
        /// Used to create interactive controls for web-based forms to accept data from the user; a wide variety of types of input data and control widgets are available, depending on the device and user agent. The <input> element is one of the most powerful and complex in all of HTML due to the sheer number of combinations of input types and attributes.
        input impl Input {
            pub const type_: Attribute = ();
            pub const name: Attribute = ();
            pub const value: Attribute = ();
            pub const checked: Attribute = ();
            pub const disabled: Attribute = ();
            pub const placeholder: Attribute = ();
            pub const required: Attribute = ();
            pub const autofocus: Attribute = ();
            pub const pattern: Attribute = ();
            pub const min: Attribute = ();
            pub const max: Attribute = ();
            pub const step: Attribute = ();
        },
        /// Represents a range of text that has been added to a document. You can use the <del> element to similarly represent a range of text that has been deleted from the document.
        ins impl Ins {
            pub const cite: Attribute = ();
            pub const datetime: Attribute = ();
        },
        /// Represents a span of inline text denoting textual user input from a keyboard, voice input, or any other text entry device. By convention, the user agent defaults to rendering the contents of a <kbd> element using its default monospace font, although this is not mandated by the HTML standard.
        kbd impl Kbd {},
        /// Represents a caption for an item in a user interface.
        label impl Label {
            pub const r#for: Attribute = ();
        },
        /// Represents a caption for the content of its parent <fieldset>.
        legend impl Legend {},
        /// Represents an item in a list. It must be contained in a parent element: an ordered list (<ol>), an unordered list (<ul>), or a menu (<menu>). In menus and unordered lists, list items are usually displayed using bullet points. In ordered lists, they are usually displayed with an ascending counter on the left, such as a number or letter.
        li impl Li {
            pub const value: Attribute = ();
        },
        /// Specifies relationships between the current document and an external resource. This element is most commonly used to link to CSS but is also used to establish site icons (both "favicon" style icons and icons for the home screen and apps on mobile devices) among other things.
        link impl Link {
            pub const crossorigin: Attribute = ();
            pub const href: Attribute = ();
            pub const rel: Attribute = ();
            pub const r#type: Attribute = ();
            pub const sizes: Attribute = ();
        },
        /// Represents the dominant content of the body of a document. The main content area consists of content that is directly related to or expands upon the central topic of a document, or the central functionality of an application.
        main impl Main {},
        /// Used with <area> elements to define an image map (a clickable link area).
        map impl Map {
            pub const name: Attribute = ();
        },
        /// Represents text which is marked or highlighted for reference or notation purposes due to the marked passage's relevance in the enclosing context.
        mark impl Mark {},
        /// The top-level element in MathML. Every valid MathML instance must be wrapped in it. In addition, you must not nest a second <math> element in another, but you can have an arbitrary number of other child elements in it.
        math impl Math {},
        /// A semantic alternative to <ul>, but treated by browsers (and exposed through the accessibility tree) as no different than <ul>. It represents an unordered list of items (which are represented by <li> elements).
        menu impl Menu {},
        /// Represents metadata that cannot be represented by other HTML meta-related elements, like <base>, <link>, <script>, <style> and <title>.
        meta impl Meta {
            pub const name: Attribute = ();
            pub const content: Attribute = ();
            pub const http_equiv: Attribute = ();
            pub const charset: Attribute = ();
        },
        /// Represents either a scalar value within a known range or a fractional value.
        meter impl Meter {
            pub const value: Attribute = ();
            pub const min: Attribute = ();
            pub const max: Attribute = ();
            pub const low: Attribute = ();
            pub const high: Attribute = ();
            pub const optimum: Attribute = ();
        },
        /// Represents a section of a page whose purpose is to provide navigation links, either within the current document or to other documents. Common examples of navigation sections are menus, tables of contents, and indexes.
        nav impl Nav {},
        /// Defines a section of HTML to be inserted if a script type on the page is unsupported or if scripting is currently turned off in the browser.
        noscript impl Noscript {},
        /// Represents an external resource, which can be treated as an image, a nested browsing context, or a resource to be handled by a plugin.
        object impl Object {
            pub const data: Attribute = ();
            pub const r#type: Attribute = ();
            pub const width: Attribute = ();
            pub const height: Attribute = ();
        },
        /// Represents an ordered list of items — typically rendered as a numbered list.
        ol impl Ol {},
        /// Creates a grouping of options within a <select> element.
        optgroup impl Optgroup {
            pub const label: Attribute = ();
        },
        /// Used to define an item contained in a <select>, an <optgroup>, or a <datalist> element. As such, <option> can represent menu items in popups and other lists of items in an HTML document.
        option impl Option {
            pub const value: Attribute = ();
            pub const disabled: Attribute = ();
            pub const selected: Attribute = ();
        },
        /// Container element into which a site or app can inject the results of a calculation or the outcome of a user action.
        output impl Output {
            pub const for_: Attribute = ();
            pub const name: Attribute = ();
        },
        /// Represents a paragraph. Paragraphs are usually represented in visual media as blocks of text separated from adjacent blocks by blank lines and/or first-line indentation, but HTML paragraphs can be any structural grouping of related content, such as images or form fields.
        p impl P {},
        /// Contains zero or more <source> elements and one <img> element to offer alternative versions of an image for different display/device scenarios.
        picture impl Picture {},
        /// Represents preformatted text which is to be presented exactly as written in the HTML file. The text is typically rendered using a non-proportional, or monospaced, font. Whitespace inside this element is displayed as written.
        pre impl Pre {},
        /// Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.
        progress impl Progress {
            pub const value: Attribute = ();
            pub const max: Attribute = ();
        },
        /// Indicates that the enclosed text is a short inline quotation. Most modern browsers implement this by surrounding the text in quotation marks. This element is intended for short quotations that don't require paragraph breaks; for long quotations use the <blockquote> element.
        q impl Q {
            pub const cite: Attribute = ();
        },
        /// Used to provide fall-back parentheses for browsers that do not support the display of ruby annotations using the <ruby> element. One <rp> element should enclose each of the opening and closing parentheses that wrap the <rt> element that contains the annotation's text.
        rp impl Rp {},
        /// Specifies the ruby text component of a ruby annotation, which is used to provide pronunciation, translation, or transliteration information for East Asian typography. The <rt> element must always be contained within a <ruby> element.
        rt impl Rt {},
        /// Represents small annotations that are rendered above, below, or next to base text, usually used for showing the pronunciation of East Asian characters. It can also be used for annotating other kinds of text, but this usage is less common.
        ruby impl Ruby {},
        /// Renders text with a strikethrough, or a line through it. Use the <s> element to represent things that are no longer relevant or no longer accurate. However, <s> is not appropriate when indicating document edits; for that, use the <del> and <ins> elements, as appropriate.
        s impl S {},
        /// Used to enclose inline text which represents sample (or quoted) output from a computer program. Its contents are typically rendered using the browser's default monospaced font (such as Courier or Lucida Console).
        samp impl Samp {},
        /// Used to embed executable code or data; this is typically used to embed or refer to JavaScript code. The <script> element can also be used with other languages, such as WebGL's GLSL shader programming language and JSON.
        script impl Script {
            pub const src: Attribute = ();
            pub const r#type: Attribute = ();
            pub const r#async: Attribute = ();
            pub const defer: Attribute = ();
            pub const crossorigin: Attribute = ();
        },
        /// Represents a part that contains a set of form controls or other content related to performing a search or filtering operation.
        search impl Search {},
        /// Represents a generic standalone section of a document, which doesn't have a more specific semantic element to represent it. Sections should always have a heading, with very few exceptions.
        section impl Section {},
        /// Represents a control that provides a menu of options.
        select impl Select {
            pub const name: Attribute = ();
            pub const multiple: Attribute = ();
            pub const size: Attribute = ();
        },
        /// Displays the content of the currently selected <option> inside a closed <select> element.
        selectedcontent impl Selectedcontent {},
        /// Part of the Web Components technology suite, this element is a placeholder inside a web component that you can fill with your own markup, which lets you create separate DOM trees and present them together.
        slot impl Slot {},
        /// Represents side-comments and small print, like copyright and legal text, independent of its styled presentation. By default, it renders text within it one font size smaller, such as from small to x-small.
        small impl Small {},
        /// Specifies multiple media resources for the picture, the audio element, or the video element. It is a void element, meaning that it has no content and does not have a closing tag. It is commonly used to offer the same media content in multiple file formats in order to provide compatibility with a broad range of browsers given their differing support for image file formats and media file formats.
        source impl Source {
            pub const src: Attribute = ();
            pub const r#type: Attribute = ();
            pub const media: Attribute = ();
            pub const sizes: Attribute = ();
        },
        /// A generic inline container for phrasing content, which does not inherently represent anything. It can be used to group elements for styling purposes (using the class or id attributes), or because they share attribute values, such as lang. It should be used only when no other semantic element is appropriate. <span> is very much like a div element, but div is a block-level element whereas a <span> is an inline-level element.
        span impl Span {},
        /// Indicates that its contents have strong importance, seriousness, or urgency. Browsers typically render the contents in bold type.
        strong impl Strong {},
        /// Contains style information for a document or part of a document. It contains CSS, which is applied to the contents of the document containing this element.
        style impl Style {},
        /// Specifies inline text which should be displayed as subscript for solely typographical reasons. Subscripts are typically rendered with a lowered baseline using smaller text.
        sub impl Sub {},
        /// Specifies a summary, caption, or legend for a details element's disclosure box. Clicking the <summary> element toggles the state of the parent <details> element open and closed.
        summary impl Summary {},
        /// Specifies inline text which is to be displayed as superscript for solely typographical reasons. Superscripts are usually rendered with a raised baseline using smaller text.
        sup impl Sup {},
        /// Container defining a new coordinate system and viewport. It is used as the outermost element of SVG documents, but it can also be used to embed an SVG fragment inside an SVG or HTML document.
        svg impl Svg {},
        /// Represents tabular data—that is, information presented in a two-dimensional table comprised of rows and columns of cells containing data.
        table impl Table {},
        /// Encapsulates a set of table rows (<tr> elements), indicating that they comprise the body of a table's (main) data.
        tbody impl Tbody {},
        /// A child of the <tr> element, it defines a cell of a table that contains data.
        td impl Td {},
        /// A mechanism for holding HTML that is not to be rendered immediately when a page is loaded but may be instantiated subsequently during runtime using JavaScript.<textarea> 	Represents a multi-line plain-text editing control, useful when you want to allow users to enter a sizeable amount of free-form text, for example, a comment on a review or feedback form.
        template impl Template {},
        /// Encapsulates a set of table rows (<tr> elements), indicating that they comprise the foot of a table with information about the table's columns. This is usually a summary of the columns, e.g., a sum of the given numbers in a column.
        tfoot impl Tfoot {},
        /// A child of the <tr> element, it defines a cell as the header of a group of table cells. The nature of this group can be explicitly defined by the scope and headers attributes.
        th impl Th {},
        /// Encapsulates a set of table rows (<tr> elements), indicating that they comprise the head of a table with information about the table's columns. This is usually in the form of column headers (<th> elements).
        thead impl Thead {},
        /// Represents a specific period in time. It may include the datetime attribute to translate dates into machine-readable format, allowing for better search engine results or custom features such as reminders.
        time impl Time {
            pub const datetime: Attribute = ();
        },
        /// Defines the document's title that is shown in a browser's title bar or a page's tab. It only contains text; HTML tags within the element, if any, are also treated as plain text.
        title impl Title {},
        /// Defines a row of cells in a table. The row's cells can then be established using a mix of <td> (data cell) and <th> (header cell) elements.
        tr impl Tr {},
        /// Used as a child of the media elements, audio and video. It lets you specify timed text tracks (or time-based data), for example to automatically handle subtitles. The tracks are formatted in WebVTT format (.vtt files)—Web Video Text Tracks.
        track impl Track {
            pub const src: Attribute = ();
            pub const kind: Attribute = ();
            pub const srclang: Attribute = ();
            pub const label: Attribute = ();
            pub const r#default: Attribute = ();
        },
        /// Represents a span of inline text which should be rendered in a way that indicates that it has a non-textual annotation. This is rendered by default as a single solid underline but may be altered using CSS.
        u impl U {},
        /// Represents an unordered list of items, typically rendered as a bulleted list.
        ul impl Ul {},
        /// Represents the name of a variable in a mathematical expression or a programming context. It's typically presented using an italicized version of the current typeface, although that behavior is browser-dependent.
        var impl Var {},
        /// Embeds a media player which supports video playback into the document. You can also use <video> for audio content, but the audio element may provide a more appropriate user experience.
        video impl Video {
            pub const autoplay: Attribute = ();
            pub const controls: Attribute = ();
            pub const r#loop: Attribute = ();
            pub const muted: Attribute = ();
            pub const preload: Attribute = ();
            pub const poster: Attribute = ();
            pub const src: Attribute = ();
        },
        /// Represents a word break opportunity—a position within text where the browser may optionally break a line, though its line-breaking rules would not otherwise create a break at that location.
        wbr impl Wbr {},
    }
}

pub mod svg {}
