use dioxus_heroicons::{solid, outline};

pub fn get_solid_icon(name: &str) -> Option<solid::Shape> {
    match name {
        "academic-cap" => Some(solid::Shape::AcademicCap),
        "adjustments" => Some(solid::Shape::Adjustments),
        "annotation" => Some(solid::Shape::Annotation),
        "archive" => Some(solid::Shape::Archive),
        "arrow-circle-down" => Some(solid::Shape::ArrowCircleDown),
        "arrow-circle-left" => Some(solid::Shape::ArrowCircleLeft),
        "arrow-circle-right" => Some(solid::Shape::ArrowCircleRight),
        "arrow-circle-up" => Some(solid::Shape::ArrowCircleUp),
        "arrow-down" => Some(solid::Shape::ArrowDown),
        "arrow-left" => Some(solid::Shape::ArrowLeft),
        "arrow-narrow-down" => Some(solid::Shape::ArrowNarrowDown),
        "arrow-narrow-left" => Some(solid::Shape::ArrowNarrowLeft),
        "arrow-narrow-right" => Some(solid::Shape::ArrowNarrowRight),
        "arrow-narrow-up" => Some(solid::Shape::ArrowNarrowUp),
        "arrow-right" => Some(solid::Shape::ArrowRight),
        "arrow-sm-down" => Some(solid::Shape::ArrowSmDown),
        "arrow-sm-left" => Some(solid::Shape::ArrowSmLeft),
        "arrow-sm-right" => Some(solid::Shape::ArrowSmRight),
        "arrow-sm-up" => Some(solid::Shape::ArrowSmUp),
        "arrow-up" => Some(solid::Shape::ArrowUp),
        "arrows-expand" => Some(solid::Shape::ArrowsExpand),
        "at-symbol" => Some(solid::Shape::AtSymbol),
        "backspace" => Some(solid::Shape::Backspace),
        "badge-check" => Some(solid::Shape::BadgeCheck),
        "ban" => Some(solid::Shape::Ban),
        "beaker" => Some(solid::Shape::Beaker),
        "bell" => Some(solid::Shape::Bell),
        "book-open" => Some(solid::Shape::BookOpen),
        "bookmark-alt" => Some(solid::Shape::BookmarkAlt),
        "bookmark" => Some(solid::Shape::Bookmark),
        "briefcase" => Some(solid::Shape::Briefcase),
        "cake" => Some(solid::Shape::Cake),
        "calculator" => Some(solid::Shape::Calculator),
        "camera" => Some(solid::Shape::Camera),
        "cash" => Some(solid::Shape::Cash),
        "chart-bar" => Some(solid::Shape::ChartBar),
        "chart-pie" => Some(solid::Shape::ChartPie),
        "chart-square-bar" => Some(solid::Shape::ChartSquareBar),
        "chat-alt-2" => Some(solid::Shape::ChatAlt2),
        "chat-alt" => Some(solid::Shape::ChatAlt),
        "chat" => Some(solid::Shape::Chat),
        "check-circle" => Some(solid::Shape::CheckCircle),
        "check" => Some(solid::Shape::Check),
        "chevron-double-down" => Some(solid::Shape::ChevronDoubleDown),
        "chevron-double-left" => Some(solid::Shape::ChevronDoubleLeft),
        "chevron-double-right" => Some(solid::Shape::ChevronDoubleRight),
        "chevron-double-up" => Some(solid::Shape::ChevronDoubleUp),
        "chevron-down" => Some(solid::Shape::ChevronDown),
        "chevron-left" => Some(solid::Shape::ChevronLeft),
        "chevron-right" => Some(solid::Shape::ChevronRight),
        "chevron-up" => Some(solid::Shape::ChevronUp),
        "chip" => Some(solid::Shape::Chip),
        "clipboard-check" => Some(solid::Shape::ClipboardCheck),
        "clipboard-copy" => Some(solid::Shape::ClipboardCopy),
        "clipboard-list" => Some(solid::Shape::ClipboardList),
        "clipboard" => Some(solid::Shape::Clipboard),
        "clock" => Some(solid::Shape::Clock),
        "cloud-download" => Some(solid::Shape::CloudDownload),
        "cloud-upload" => Some(solid::Shape::CloudUpload),
        "cloud" => Some(solid::Shape::Cloud),
        "code" => Some(solid::Shape::Code),
        "cog" => Some(solid::Shape::Cog),
        "collection" => Some(solid::Shape::Collection),
        "color-swatch" => Some(solid::Shape::ColorSwatch),
        "credit-card" => Some(solid::Shape::CreditCard),
        "cube-transparent" => Some(solid::Shape::CubeTransparent),
        "cube" => Some(solid::Shape::Cube),
        "currency-bangladeshi" => Some(solid::Shape::CurrencyBangladeshi),
        "currency-dollar" => Some(solid::Shape::CurrencyDollar),
        "currency-euro" => Some(solid::Shape::CurrencyEuro),
        "currency-pound" => Some(solid::Shape::CurrencyPound),
        "currency-rupee" => Some(solid::Shape::CurrencyRupee),
        "currency-yen" => Some(solid::Shape::CurrencyYen),
        "cursor-click" => Some(solid::Shape::CursorClick),
        "database" => Some(solid::Shape::Database),
        "desktop-computer" => Some(solid::Shape::DesktopComputer),
        "device-mobile" => Some(solid::Shape::DeviceMobile),
        "device-tablet" => Some(solid::Shape::DeviceTablet),
        "document-add" => Some(solid::Shape::DocumentAdd),
        "document-download" => Some(solid::Shape::DocumentDownload),
        "document-duplicate" => Some(solid::Shape::DocumentDuplicate),
        "document-remove" => Some(solid::Shape::DocumentRemove),
        "document-report" => Some(solid::Shape::DocumentReport),
        "document-search" => Some(solid::Shape::DocumentSearch),
        "document-text" => Some(solid::Shape::DocumentText),
        "document" => Some(solid::Shape::Document),
        "dots-circle-horizontal" => Some(solid::Shape::DotsCircleHorizontal),
        "dots-horizontal" => Some(solid::Shape::DotsHorizontal),
        "dots-vertical" => Some(solid::Shape::DotsVertical),
        "download" => Some(solid::Shape::Download),
        "duplicate" => Some(solid::Shape::Duplicate),
        "emoji-happy" => Some(solid::Shape::EmojiHappy),
        "emoji-sad" => Some(solid::Shape::EmojiSad),
        "exclamation-circle" => Some(solid::Shape::ExclamationCircle),
        "exclamation" => Some(solid::Shape::Exclamation),
        "external-link" => Some(solid::Shape::ExternalLink),
        "eye-off" => Some(solid::Shape::EyeOff),
        "eye" => Some(solid::Shape::Eye),
        "fast-forward" => Some(solid::Shape::FastForward),
        "film" => Some(solid::Shape::Film),
        "filter" => Some(solid::Shape::Filter),
        "finger-print" => Some(solid::Shape::FingerPrint),
        "fire" => Some(solid::Shape::Fire),
        "flag" => Some(solid::Shape::Flag),
        "folder-add" => Some(solid::Shape::FolderAdd),
        "folder-download" => Some(solid::Shape::FolderDownload),
        "folder-open" => Some(solid::Shape::FolderOpen),
        "folder-remove" => Some(solid::Shape::FolderRemove),
        "folder" => Some(solid::Shape::Folder),
        "gift" => Some(solid::Shape::Gift),
        "globe-alt" => Some(solid::Shape::GlobeAlt),
        "globe" => Some(solid::Shape::Globe),
        "hand" => Some(solid::Shape::Hand),
        "hashtag" => Some(solid::Shape::Hashtag),
        "heart" => Some(solid::Shape::Heart),
        "home" => Some(solid::Shape::Home),
        "identification" => Some(solid::Shape::Identification),
        "inbox-in" => Some(solid::Shape::InboxIn),
        "inbox" => Some(solid::Shape::Inbox),
        "information-circle" => Some(solid::Shape::InformationCircle),
        "key" => Some(solid::Shape::Key),
        "library" => Some(solid::Shape::Library),
        "light-bulb" => Some(solid::Shape::LightBulb),
        "lightning-bolt" => Some(solid::Shape::LightningBolt),
        "link" => Some(solid::Shape::Link),
        "location-marker" => Some(solid::Shape::LocationMarker),
        "lock-closed" => Some(solid::Shape::LockClosed),
        "lock-open" => Some(solid::Shape::LockOpen),
        "login" => Some(solid::Shape::Login),
        "logout" => Some(solid::Shape::Logout),
        "mail-open" => Some(solid::Shape::MailOpen),
        "mail" => Some(solid::Shape::Mail),
        "map" => Some(solid::Shape::Map),
        "menu-alt-1" => Some(solid::Shape::MenuAlt1),
        "menu-alt-2" => Some(solid::Shape::MenuAlt2),
        "menu-alt-3" => Some(solid::Shape::MenuAlt3),
        "menu-alt-4" => Some(solid::Shape::MenuAlt4),
        "menu" => Some(solid::Shape::Menu),
        "microphone" => Some(solid::Shape::Microphone),
        "minus-circle" => Some(solid::Shape::MinusCircle),
        "minus-sm" => Some(solid::Shape::MinusSm),
        "minus" => Some(solid::Shape::Minus),
        "moon" => Some(solid::Shape::Moon),
        "music-note" => Some(solid::Shape::MusicNote),
        "newspaper" => Some(solid::Shape::Newspaper),
        "office-building" => Some(solid::Shape::OfficeBuilding),
        "paper-airplane" => Some(solid::Shape::PaperAirplane),
        "paper-clip" => Some(solid::Shape::PaperClip),
        "pause" => Some(solid::Shape::Pause),
        "pencil-alt" => Some(solid::Shape::PencilAlt),
        "pencil" => Some(solid::Shape::Pencil),
        "phone-incoming" => Some(solid::Shape::PhoneIncoming),
        "phone-missed-call" => Some(solid::Shape::PhoneMissedCall),
        "phone-outgoing" => Some(solid::Shape::PhoneOutgoing),
        "phone" => Some(solid::Shape::Phone),
        "photograph" => Some(solid::Shape::Photograph),
        "play" => Some(solid::Shape::Play),
        "plus-circle" => Some(solid::Shape::PlusCircle),
        "plus-sm" => Some(solid::Shape::PlusSm),
        "plus" => Some(solid::Shape::Plus),
        "presentation-chart-bar" => Some(solid::Shape::PresentationChartBar),
        "presentation-chart-line" => Some(solid::Shape::PresentationChartLine),
        "printer" => Some(solid::Shape::Printer),
        "puzzle" => Some(solid::Shape::Puzzle),
        "qrcode" => Some(solid::Shape::Qrcode),
        "question-mark-circle" => Some(solid::Shape::QuestionMarkCircle),
        "receipt-refund" => Some(solid::Shape::ReceiptRefund),
        "receipt-tax" => Some(solid::Shape::ReceiptTax),
        "refresh" => Some(solid::Shape::Refresh),
        "reply" => Some(solid::Shape::Reply),
        "rewind" => Some(solid::Shape::Rewind),
        "rss" => Some(solid::Shape::Rss),
        "save-as" => Some(solid::Shape::SaveAs),
        "save" => Some(solid::Shape::Save),
        "scale" => Some(solid::Shape::Scale),
        "scissors" => Some(solid::Shape::Scissors),
        "search-circle" => Some(solid::Shape::SearchCircle),
        "search" => Some(solid::Shape::Search),
        "selector" => Some(solid::Shape::Selector),
        "server" => Some(solid::Shape::Server),
        "share" => Some(solid::Shape::Share),
        "shield-check" => Some(solid::Shape::ShieldCheck),
        "shield-exclamation" => Some(solid::Shape::ShieldExclamation),
        "shopping-bag" => Some(solid::Shape::ShoppingBag),
        "shopping-cart" => Some(solid::Shape::ShoppingCart),
        "sort-ascending" => Some(solid::Shape::SortAscending),
        "sort-descending" => Some(solid::Shape::SortDescending),
        "sparkles" => Some(solid::Shape::Sparkles),
        "speakerphone" => Some(solid::Shape::Speakerphone),
        "star" => Some(solid::Shape::Star),
        "status-offline" => Some(solid::Shape::StatusOffline),
        "status-online" => Some(solid::Shape::StatusOnline),
        "stop" => Some(solid::Shape::Stop),
        "sun" => Some(solid::Shape::Sun),
        "support" => Some(solid::Shape::Support),
        "switch-horizontal" => Some(solid::Shape::SwitchHorizontal),
        "switch-vertical" => Some(solid::Shape::SwitchVertical),
        "table" => Some(solid::Shape::Table),
        "tag" => Some(solid::Shape::Tag),
        "template" => Some(solid::Shape::Template),
        "terminal" => Some(solid::Shape::Terminal),
        "thumb-down" => Some(solid::Shape::ThumbDown),
        "thumb-up" => Some(solid::Shape::ThumbUp),
        "ticket" => Some(solid::Shape::Ticket),
        "translate" => Some(solid::Shape::Translate),
        "trash" => Some(solid::Shape::Trash),
        "trending-down" => Some(solid::Shape::TrendingDown),
        "trending-up" => Some(solid::Shape::TrendingUp),
        "truck" => Some(solid::Shape::Truck),
        "upload" => Some(solid::Shape::Upload),
        "user-add" => Some(solid::Shape::UserAdd),
        "user-circle" => Some(solid::Shape::UserCircle),
        "user-group" => Some(solid::Shape::UserGroup),
        "user-remove" => Some(solid::Shape::UserRemove),
        "user" => Some(solid::Shape::User),
        "users" => Some(solid::Shape::Users),
        "variable" => Some(solid::Shape::Variable),
        "video-camera" => Some(solid::Shape::VideoCamera),
        "view-boards" => Some(solid::Shape::ViewBoards),
        "view-grid-add" => Some(solid::Shape::ViewGridAdd),
        "view-grid" => Some(solid::Shape::ViewGrid),
        "view-list" => Some(solid::Shape::ViewList),
        "volume-off" => Some(solid::Shape::VolumeOff),
        "volume-up" => Some(solid::Shape::VolumeUp),
        "wifi" => Some(solid::Shape::Wifi),
        "x-circle" => Some(solid::Shape::XCircle),
        "x" => Some(solid::Shape::X),
        "zoom-in" => Some(solid::Shape::ZoomIn),
        "zoom-out" => Some(solid::Shape::ZoomOut),
        _ => None
    }
}

pub fn get_outline_icon(name: &str) -> Option<outline::Shape> {
    match name {
        "academic-cap" => Some(outline::Shape::AcademicCap),
        "adjustments" => Some(outline::Shape::Adjustments),
        "annotation" => Some(outline::Shape::Annotation),
        "archive" => Some(outline::Shape::Archive),
        "arrow-circle-down" => Some(outline::Shape::ArrowCircleDown),
        "arrow-circle-left" => Some(outline::Shape::ArrowCircleLeft),
        "arrow-circle-right" => Some(outline::Shape::ArrowCircleRight),
        "arrow-circle-up" => Some(outline::Shape::ArrowCircleUp),
        "arrow-down" => Some(outline::Shape::ArrowDown),
        "arrow-left" => Some(outline::Shape::ArrowLeft),
        "arrow-narrow-down" => Some(outline::Shape::ArrowNarrowDown),
        "arrow-narrow-left" => Some(outline::Shape::ArrowNarrowLeft),
        "arrow-narrow-right" => Some(outline::Shape::ArrowNarrowRight),
        "arrow-narrow-up" => Some(outline::Shape::ArrowNarrowUp),
        "arrow-right" => Some(outline::Shape::ArrowRight),
        "arrow-sm-down" => Some(outline::Shape::ArrowSmDown),
        "arrow-sm-left" => Some(outline::Shape::ArrowSmLeft),
        "arrow-sm-right" => Some(outline::Shape::ArrowSmRight),
        "arrow-sm-up" => Some(outline::Shape::ArrowSmUp),
        "arrow-up" => Some(outline::Shape::ArrowUp),
        "arrows-expand" => Some(outline::Shape::ArrowsExpand),
        "at-symbol" => Some(outline::Shape::AtSymbol),
        "backspace" => Some(outline::Shape::Backspace),
        "badge-check" => Some(outline::Shape::BadgeCheck),
        "ban" => Some(outline::Shape::Ban),
        "beaker" => Some(outline::Shape::Beaker),
        "bell" => Some(outline::Shape::Bell),
        "book-open" => Some(outline::Shape::BookOpen),
        "bookmark-alt" => Some(outline::Shape::BookmarkAlt),
        "bookmark" => Some(outline::Shape::Bookmark),
        "briefcase" => Some(outline::Shape::Briefcase),
        "cake" => Some(outline::Shape::Cake),
        "calculator" => Some(outline::Shape::Calculator),
        "camera" => Some(outline::Shape::Camera),
        "cash" => Some(outline::Shape::Cash),
        "chart-bar" => Some(outline::Shape::ChartBar),
        "chart-pie" => Some(outline::Shape::ChartPie),
        "chart-square-bar" => Some(outline::Shape::ChartSquareBar),
        "chat-alt-2" => Some(outline::Shape::ChatAlt2),
        "chat-alt" => Some(outline::Shape::ChatAlt),
        "chat" => Some(outline::Shape::Chat),
        "check-circle" => Some(outline::Shape::CheckCircle),
        "check" => Some(outline::Shape::Check),
        "chevron-double-down" => Some(outline::Shape::ChevronDoubleDown),
        "chevron-double-left" => Some(outline::Shape::ChevronDoubleLeft),
        "chevron-double-right" => Some(outline::Shape::ChevronDoubleRight),
        "chevron-double-up" => Some(outline::Shape::ChevronDoubleUp),
        "chevron-down" => Some(outline::Shape::ChevronDown),
        "chevron-left" => Some(outline::Shape::ChevronLeft),
        "chevron-right" => Some(outline::Shape::ChevronRight),
        "chevron-up" => Some(outline::Shape::ChevronUp),
        "chip" => Some(outline::Shape::Chip),
        "clipboard-check" => Some(outline::Shape::ClipboardCheck),
        "clipboard-copy" => Some(outline::Shape::ClipboardCopy),
        "clipboard-list" => Some(outline::Shape::ClipboardList),
        "clipboard" => Some(outline::Shape::Clipboard),
        "clock" => Some(outline::Shape::Clock),
        "cloud-download" => Some(outline::Shape::CloudDownload),
        "cloud-upload" => Some(outline::Shape::CloudUpload),
        "cloud" => Some(outline::Shape::Cloud),
        "code" => Some(outline::Shape::Code),
        "cog" => Some(outline::Shape::Cog),
        "collection" => Some(outline::Shape::Collection),
        "color-swatch" => Some(outline::Shape::ColorSwatch),
        "credit-card" => Some(outline::Shape::CreditCard),
        "cube-transparent" => Some(outline::Shape::CubeTransparent),
        "cube" => Some(outline::Shape::Cube),
        "currency-bangladeshi" => Some(outline::Shape::CurrencyBangladeshi),
        "currency-dollar" => Some(outline::Shape::CurrencyDollar),
        "currency-euro" => Some(outline::Shape::CurrencyEuro),
        "currency-pound" => Some(outline::Shape::CurrencyPound),
        "currency-rupee" => Some(outline::Shape::CurrencyRupee),
        "currency-yen" => Some(outline::Shape::CurrencyYen),
        "cursor-click" => Some(outline::Shape::CursorClick),
        "database" => Some(outline::Shape::Database),
        "desktop-computer" => Some(outline::Shape::DesktopComputer),
        "device-mobile" => Some(outline::Shape::DeviceMobile),
        "device-tablet" => Some(outline::Shape::DeviceTablet),
        "document-add" => Some(outline::Shape::DocumentAdd),
        "document-download" => Some(outline::Shape::DocumentDownload),
        "document-duplicate" => Some(outline::Shape::DocumentDuplicate),
        "document-remove" => Some(outline::Shape::DocumentRemove),
        "document-report" => Some(outline::Shape::DocumentReport),
        "document-search" => Some(outline::Shape::DocumentSearch),
        "document-text" => Some(outline::Shape::DocumentText),
        "document" => Some(outline::Shape::Document),
        "dots-circle-horizontal" => Some(outline::Shape::DotsCircleHorizontal),
        "dots-horizontal" => Some(outline::Shape::DotsHorizontal),
        "dots-vertical" => Some(outline::Shape::DotsVertical),
        "download" => Some(outline::Shape::Download),
        "duplicate" => Some(outline::Shape::Duplicate),
        "emoji-happy" => Some(outline::Shape::EmojiHappy),
        "emoji-sad" => Some(outline::Shape::EmojiSad),
        "exclamation-circle" => Some(outline::Shape::ExclamationCircle),
        "exclamation" => Some(outline::Shape::Exclamation),
        "external-link" => Some(outline::Shape::ExternalLink),
        "eye-off" => Some(outline::Shape::EyeOff),
        "eye" => Some(outline::Shape::Eye),
        "fast-forward" => Some(outline::Shape::FastForward),
        "film" => Some(outline::Shape::Film),
        "filter" => Some(outline::Shape::Filter),
        "finger-print" => Some(outline::Shape::FingerPrint),
        "fire" => Some(outline::Shape::Fire),
        "flag" => Some(outline::Shape::Flag),
        "folder-add" => Some(outline::Shape::FolderAdd),
        "folder-download" => Some(outline::Shape::FolderDownload),
        "folder-open" => Some(outline::Shape::FolderOpen),
        "folder-remove" => Some(outline::Shape::FolderRemove),
        "folder" => Some(outline::Shape::Folder),
        "gift" => Some(outline::Shape::Gift),
        "globe-alt" => Some(outline::Shape::GlobeAlt),
        "globe" => Some(outline::Shape::Globe),
        "hand" => Some(outline::Shape::Hand),
        "hashtag" => Some(outline::Shape::Hashtag),
        "heart" => Some(outline::Shape::Heart),
        "home" => Some(outline::Shape::Home),
        "identification" => Some(outline::Shape::Identification),
        "inbox-in" => Some(outline::Shape::InboxIn),
        "inbox" => Some(outline::Shape::Inbox),
        "information-circle" => Some(outline::Shape::InformationCircle),
        "key" => Some(outline::Shape::Key),
        "library" => Some(outline::Shape::Library),
        "light-bulb" => Some(outline::Shape::LightBulb),
        "lightning-bolt" => Some(outline::Shape::LightningBolt),
        "link" => Some(outline::Shape::Link),
        "location-marker" => Some(outline::Shape::LocationMarker),
        "lock-closed" => Some(outline::Shape::LockClosed),
        "lock-open" => Some(outline::Shape::LockOpen),
        "login" => Some(outline::Shape::Login),
        "logout" => Some(outline::Shape::Logout),
        "mail-open" => Some(outline::Shape::MailOpen),
        "mail" => Some(outline::Shape::Mail),
        "map" => Some(outline::Shape::Map),
        "menu-alt-1" => Some(outline::Shape::MenuAlt1),
        "menu-alt-2" => Some(outline::Shape::MenuAlt2),
        "menu-alt-3" => Some(outline::Shape::MenuAlt3),
        "menu-alt-4" => Some(outline::Shape::MenuAlt4),
        "menu" => Some(outline::Shape::Menu),
        "microphone" => Some(outline::Shape::Microphone),
        "minus-circle" => Some(outline::Shape::MinusCircle),
        "minus-sm" => Some(outline::Shape::MinusSm),
        "minus" => Some(outline::Shape::Minus),
        "moon" => Some(outline::Shape::Moon),
        "music-note" => Some(outline::Shape::MusicNote),
        "newspaper" => Some(outline::Shape::Newspaper),
        "office-building" => Some(outline::Shape::OfficeBuilding),
        "paper-airplane" => Some(outline::Shape::PaperAirplane),
        "paper-clip" => Some(outline::Shape::PaperClip),
        "pause" => Some(outline::Shape::Pause),
        "pencil-alt" => Some(outline::Shape::PencilAlt),
        "pencil" => Some(outline::Shape::Pencil),
        "phone-incoming" => Some(outline::Shape::PhoneIncoming),
        "phone-missed-call" => Some(outline::Shape::PhoneMissedCall),
        "phone-outgoing" => Some(outline::Shape::PhoneOutgoing),
        "phone" => Some(outline::Shape::Phone),
        "photograph" => Some(outline::Shape::Photograph),
        "play" => Some(outline::Shape::Play),
        "plus-circle" => Some(outline::Shape::PlusCircle),
        "plus-sm" => Some(outline::Shape::PlusSm),
        "plus" => Some(outline::Shape::Plus),
        "presentation-chart-bar" => Some(outline::Shape::PresentationChartBar),
        "presentation-chart-line" => Some(outline::Shape::PresentationChartLine),
        "printer" => Some(outline::Shape::Printer),
        "puzzle" => Some(outline::Shape::Puzzle),
        "qrcode" => Some(outline::Shape::Qrcode),
        "question-mark-circle" => Some(outline::Shape::QuestionMarkCircle),
        "receipt-refund" => Some(outline::Shape::ReceiptRefund),
        "receipt-tax" => Some(outline::Shape::ReceiptTax),
        "refresh" => Some(outline::Shape::Refresh),
        "reply" => Some(outline::Shape::Reply),
        "rewind" => Some(outline::Shape::Rewind),
        "rss" => Some(outline::Shape::Rss),
        "save-as" => Some(outline::Shape::SaveAs),
        "save" => Some(outline::Shape::Save),
        "scale" => Some(outline::Shape::Scale),
        "scissors" => Some(outline::Shape::Scissors),
        "search-circle" => Some(outline::Shape::SearchCircle),
        "search" => Some(outline::Shape::Search),
        "selector" => Some(outline::Shape::Selector),
        "server" => Some(outline::Shape::Server),
        "share" => Some(outline::Shape::Share),
        "shield-check" => Some(outline::Shape::ShieldCheck),
        "shield-exclamation" => Some(outline::Shape::ShieldExclamation),
        "shopping-bag" => Some(outline::Shape::ShoppingBag),
        "shopping-cart" => Some(outline::Shape::ShoppingCart),
        "sort-ascending" => Some(outline::Shape::SortAscending),
        "sort-descending" => Some(outline::Shape::SortDescending),
        "sparkles" => Some(outline::Shape::Sparkles),
        "speakerphone" => Some(outline::Shape::Speakerphone),
        "star" => Some(outline::Shape::Star),
        "status-offline" => Some(outline::Shape::StatusOffline),
        "status-online" => Some(outline::Shape::StatusOnline),
        "stop" => Some(outline::Shape::Stop),
        "sun" => Some(outline::Shape::Sun),
        "support" => Some(outline::Shape::Support),
        "switch-horizontal" => Some(outline::Shape::SwitchHorizontal),
        "switch-vertical" => Some(outline::Shape::SwitchVertical),
        "table" => Some(outline::Shape::Table),
        "tag" => Some(outline::Shape::Tag),
        "template" => Some(outline::Shape::Template),
        "terminal" => Some(outline::Shape::Terminal),
        "thumb-down" => Some(outline::Shape::ThumbDown),
        "thumb-up" => Some(outline::Shape::ThumbUp),
        "ticket" => Some(outline::Shape::Ticket),
        "translate" => Some(outline::Shape::Translate),
        "trash" => Some(outline::Shape::Trash),
        "trending-down" => Some(outline::Shape::TrendingDown),
        "trending-up" => Some(outline::Shape::TrendingUp),
        "truck" => Some(outline::Shape::Truck),
        "upload" => Some(outline::Shape::Upload),
        "user-add" => Some(outline::Shape::UserAdd),
        "user-circle" => Some(outline::Shape::UserCircle),
        "user-group" => Some(outline::Shape::UserGroup),
        "user-remove" => Some(outline::Shape::UserRemove),
        "user" => Some(outline::Shape::User),
        "users" => Some(outline::Shape::Users),
        "variable" => Some(outline::Shape::Variable),
        "video-camera" => Some(outline::Shape::VideoCamera),
        "view-boards" => Some(outline::Shape::ViewBoards),
        "view-grid-add" => Some(outline::Shape::ViewGridAdd),
        "view-grid" => Some(outline::Shape::ViewGrid),
        "view-list" => Some(outline::Shape::ViewList),
        "volume-off" => Some(outline::Shape::VolumeOff),
        "volume-up" => Some(outline::Shape::VolumeUp),
        "wifi" => Some(outline::Shape::Wifi),
        "x-circle" => Some(outline::Shape::XCircle),
        "x" => Some(outline::Shape::X),
        "zoom-in" => Some(outline::Shape::ZoomIn),
        "zoom-out" => Some(outline::Shape::ZoomOut),
        _ => None
    }
}