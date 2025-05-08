#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{primitive::str, vec::Vec};

// Hypertext Markup Language Element Tag Definition
pub type HTMLEndTag = &'static str;

// Hypertext Markup Language Element End Tag
pub const HTML_ANCHOR_END_TAG: HTMLEndTag = "</a>";
pub const HTML_ABBREVIATION_END_TAG: HTMLEndTag = "</abbr>";
pub const HTML_CONTACT_ADDRESS_END_TAG: HTMLEndTag = "</address>";
pub const HTML_ARTICLE_END_TAG: HTMLEndTag = "</article>";
pub const HTML_ASIDE_CONTENT_END_TAG: HTMLEndTag = "</aside>";
pub const HTML_AUDIO_END_TAG: HTMLEndTag = "</audio>";
pub const HTML_BRING_ATTENTION_TO_END_TAG: HTMLEndTag = "</b>";
pub const HTML_BIDIRECTIONAL_ISOLATE_END_TAG: HTMLEndTag = "</bdi>";
pub const HTML_BIDIRECTIONAL_TEXT_OVERRIDE_END_TAG: HTMLEndTag = "</bdo>";
pub const HTML_BLOCK_QUOTATION_END_TAG: HTMLEndTag = "</blockquote>";
pub const HTML_DOCUMENT_BODY_END_TAG: HTMLEndTag = "</body>";
pub const HTML_BUTTON_END_TAG: HTMLEndTag = "</button>";
pub const HTML_GRAPHICS_CANVAS_END_TAG: HTMLEndTag = "</canvas>";
pub const HTML_TABLE_CAPTION_END_TAG: HTMLEndTag = "</caption>";
pub const HTML_CITATION_END_TAG: HTMLEndTag = "</cite>";
pub const HTML_INLINE_CODE_END_TAG: HTMLEndTag = "</code>";
pub const HTML_TABLE_COLUMN_GROUP_END_TAG: HTMLEndTag = "</colgroup>";
pub const HTML_DATA_END_TAG: HTMLEndTag = "</data>";
pub const HTML_DATA_LIST_END_TAG: HTMLEndTag = "</datalist>";
pub const HTML_DESCRIPTION_DETAILS_END_TAG: HTMLEndTag = "</dd>";
pub const HTML_DELETED_TEXT_END_TAG: HTMLEndTag = "</del>";
pub const HTML_DETAILS_DISCLOSURE_END_TAG: HTMLEndTag = "</details>";
pub const HTML_DEFINITION_END_TAG: HTMLEndTag = "</dfn>";
pub const HTML_DIALOGUE_END_TAG: HTMLEndTag = "</dialog>";
pub const HTML_CONTENT_DIVISION_END_TAG: HTMLEndTag = "</div>";
pub const HTML_DESCRIPTION_LIST_END_TAG: HTMLEndTag = "</dl>";
pub const HTML_DESCRIPTION_TERM_END_TAG: HTMLEndTag = "</dt>";
pub const HTML_EMPHASIS_END_TAG: HTMLEndTag = "</em>";
pub const HTML_FIELD_SET_END_TAG: HTMLEndTag = "</fieldset>";
pub const HTML_FIGURE_CAPTION_END_TAG: HTMLEndTag = "</figcaption>";
pub const HTML_FIGURE_END_TAG: HTMLEndTag = "</figure>";
pub const HTML_FOOTER_END_TAG: HTMLEndTag = "</footer>";
pub const HTML_FORM_END_TAG: HTMLEndTag = "</form>";
pub const HTML_SECTION_HEADING_ONE_END_TAG: HTMLEndTag = "</h1>";
pub const HTML_SECTION_HEADING_TWO_END_TAG: HTMLEndTag = "</h2>";
pub const HTML_SECTION_HEADING_THREE_END_TAG: HTMLEndTag = "</h3>";
pub const HTML_SECTION_HEADING_FOUR_END_TAG: HTMLEndTag = "</h4>";
pub const HTML_SECTION_HEADING_FIVE_END_TAG: HTMLEndTag = "</h5>";
pub const HTML_SECTION_HEADING_SIX_END_TAG: HTMLEndTag = "</h6>";
pub const HTML_DOCUMENT_METADATA_HEADER_END_TAG: HTMLEndTag = "</head>";
pub const HTML_HEADER_END_TAG: HTMLEndTag = "</header>";
pub const HTML_HEADING_GROUP_END_TAG: HTMLEndTag = "</hgroup>";
pub const HTML_DOCUMENT_ROOT_END_TAG: HTMLEndTag = "</html>";
pub const HTML_IDIOMATIC_TEXT_END_TAG: HTMLEndTag = "</i>";
pub const HTML_INLINE_FRAME_END_TAG: HTMLEndTag = "</iframe>";
pub const HTML_INSERTED_TEXT_END_TAG: HTMLEndTag = "</ins>";
pub const HTML_KEYBOARD_INSET_END_TAG: HTMLEndTag = "</kbd>";
pub const HTML_LABEL_END_TAG: HTMLEndTag = "</label>";
pub const HTML_FIELD_SET_LEGEND_END_TAG: HTMLEndTag = "</legend>";
pub const HTML_LIST_ITEM_END_TAG: HTMLEndTag = "</li>";
pub const HTML_MAIN_END_TAG: HTMLEndTag = "</main>";
pub const HTML_IMAGE_MAP_END_TAG: HTMLEndTag = "</map>";
pub const HTML_MARK_TEXT_END_TAG: HTMLEndTag = "</mark>";
pub const HTML_MENU_END_TAG: HTMLEndTag = "</menu>";
pub const HTML_METER_END_TAG: HTMLEndTag = "</meter>";
pub const HTML_NAVIGATION_SECTION_END_TAG: HTMLEndTag = "</nav>";
pub const HTML_NO_SCRIPT_END_TAG: HTMLEndTag = "</noscript>";
pub const HTML_EXTERNAL_OBJECT_END_TAG: HTMLEndTag = "</object>";
pub const HTML_ORDERED_LIST_END_TAG: HTMLEndTag = "</ol>";
pub const HTML_OPTION_GROUP_END_TAG: HTMLEndTag = "</optgroup>";
pub const HTML_OPTION_END_TAG: HTMLEndTag = "</option>";
pub const HTML_OUTPUT_END_TAG: HTMLEndTag = "</output>";
pub const HTML_PARAGRAPH_END_TAG: HTMLEndTag = "</p>";
pub const HTML_PICTURE_END_TAG: HTMLEndTag = "</picture>";
pub const HTML_PREFORMATED_TEXT_END_TAG: HTMLEndTag = "</pre>";
pub const HTML_PROGRESS_INDIDCATOR_END_TAG: HTMLEndTag = "</progress>";
pub const HTML_INLINE_QUOTATION_END_TAG: HTMLEndTag = "</q>";
pub const HTML_RUBY_FALLBACK_PARENTHESIS_END_TAG: HTMLEndTag = "</rp>";
pub const HTML_RUBY_TEXT_END_TAG: HTMLEndTag = "</rt>";
pub const HTML_RUBY_ANNOTATION_END_TAG: HTMLEndTag = "</ruby>";
pub const HTML_STRIKE_THROUGH_END_TAG: HTMLEndTag = "</s>";
pub const HTML_SAMPLE_OUTPUT_END_TAG: HTMLEndTag = "</samp>";
pub const HTML_SCRIPT_END_TAG: HTMLEndTag = "</script>";
pub const HTML_GENERIC_SEARCH_END_TAG: HTMLEndTag = "</search>";
pub const HTML_GENERIC_SECTION_END_TAG: HTMLEndTag = "</section>";
pub const HTML_SELECT_END_TAG: HTMLEndTag = "</select>";
pub const HTML_WEB_COMPONENT_SLOT_END_TAG: HTMLEndTag = "</slot>";
pub const HTML_SIDE_COMMENT_SMALL_PRINT_END_TAG: HTMLEndTag = "</small>";
pub const HTML_CONTENT_SPAN_END_TAG: HTMLEndTag = "</span>";
pub const HTML_STRONG_IMPORTANCE_END_TAG: HTMLEndTag = "</strong>";
pub const HTML_STYLE_INFORMATION_END_TAG: HTMLEndTag = "</style>";
pub const HTML_SUBSCRIPT_END_TAG: HTMLEndTag = "</sub>";
pub const HTML_DISCLOSURE_SUMMARY_END_TAG: HTMLEndTag = "</summary>";
pub const HTML_SUPERSCRIPT_END_TAG: HTMLEndTag = "</sup>";
pub const HTML_TABLE_END_TAG: HTMLEndTag = "</table>";
pub const HTML_TABLE_BODY_END_TAG: HTMLEndTag = "</tbody>";
pub const HTML_TABLE_DATA_CELL_END_TAG: HTMLEndTag = "</td>";
pub const HTML_CONTENT_TEMPLATE_END_TAG: HTMLEndTag = "</template>";
pub const HTML_TEXT_AREA_END_TAG: HTMLEndTag = "</textarea>";
pub const HTML_TABLE_FOOTER_END_TAG: HTMLEndTag = "</tfoot>";
pub const HTML_TABLE_HEADER_END_TAG: HTMLEndTag = "</th>";
pub const HTML_TABLE_HEAD_END_TAG: HTMLEndTag = "</thead>";
pub const HTML_DATE_TIME_END_TAG: HTMLEndTag = "</time>";
pub const HTML_DOCUMENT_TITLE_END_TAG: HTMLEndTag = "</title>";
pub const HTML_TABLE_ROW_END_TAG: HTMLEndTag = "</tr>";
pub const HTML_UNARTICULATED_ANNOTATION_END_TAG: HTMLEndTag = "</u>";
pub const HTML_UNORDERED_LIST_END_TAG: HTMLEndTag = "</ul>";
pub const HTML_VARIABLE_END_TAG: HTMLEndTag = "</var>";
pub const HTML_VIDEO_EMBED_END_TAG: HTMLEndTag = "</video>";

// Hypertext Markup Language End Tag Vector
pub fn html_end_tags() -> Vec<HTMLEndTag> {
    let hypertext_markup_end_tags: Vec<HTMLEndTag> = Vec::from([
        HTML_ANCHOR_END_TAG,
        HTML_ABBREVIATION_END_TAG,
        HTML_CONTACT_ADDRESS_END_TAG,
        HTML_ARTICLE_END_TAG,
        HTML_ASIDE_CONTENT_END_TAG,
        HTML_AUDIO_END_TAG,
        HTML_BRING_ATTENTION_TO_END_TAG,
        HTML_BIDIRECTIONAL_ISOLATE_END_TAG,
        HTML_BIDIRECTIONAL_TEXT_OVERRIDE_END_TAG,
        HTML_BLOCK_QUOTATION_END_TAG,
        HTML_DOCUMENT_BODY_END_TAG,
        HTML_BUTTON_END_TAG,
        HTML_GRAPHICS_CANVAS_END_TAG,
        HTML_TABLE_CAPTION_END_TAG,
        HTML_CITATION_END_TAG,
        HTML_INLINE_CODE_END_TAG,
        HTML_TABLE_COLUMN_GROUP_END_TAG,
        HTML_DATA_END_TAG,
        HTML_DATA_LIST_END_TAG,
        HTML_DESCRIPTION_DETAILS_END_TAG,
        HTML_DELETED_TEXT_END_TAG,
        HTML_DETAILS_DISCLOSURE_END_TAG,
        HTML_DEFINITION_END_TAG,
        HTML_DIALOGUE_END_TAG,
        HTML_CONTENT_DIVISION_END_TAG,
        HTML_DESCRIPTION_LIST_END_TAG,
        HTML_DESCRIPTION_TERM_END_TAG,
        HTML_EMPHASIS_END_TAG,
        HTML_FIELD_SET_END_TAG,
        HTML_FIGURE_CAPTION_END_TAG,
        HTML_FIGURE_END_TAG,
        HTML_FOOTER_END_TAG,
        HTML_FORM_END_TAG,
        HTML_SECTION_HEADING_ONE_END_TAG,
        HTML_SECTION_HEADING_TWO_END_TAG,
        HTML_SECTION_HEADING_THREE_END_TAG,
        HTML_SECTION_HEADING_FOUR_END_TAG,
        HTML_SECTION_HEADING_FIVE_END_TAG,
        HTML_SECTION_HEADING_SIX_END_TAG,
        HTML_DOCUMENT_METADATA_HEADER_END_TAG,
        HTML_HEADER_END_TAG,
        HTML_HEADING_GROUP_END_TAG,
        HTML_DOCUMENT_ROOT_END_TAG,
        HTML_IDIOMATIC_TEXT_END_TAG,
        HTML_INLINE_FRAME_END_TAG,
        HTML_INSERTED_TEXT_END_TAG,
        HTML_KEYBOARD_INSET_END_TAG,
        HTML_LABEL_END_TAG,
        HTML_FIELD_SET_LEGEND_END_TAG,
        HTML_LIST_ITEM_END_TAG,
        HTML_MAIN_END_TAG,
        HTML_IMAGE_MAP_END_TAG,
        HTML_MARK_TEXT_END_TAG,
        HTML_MENU_END_TAG,
        HTML_METER_END_TAG,
        HTML_NAVIGATION_SECTION_END_TAG,
        HTML_NO_SCRIPT_END_TAG,
        HTML_EXTERNAL_OBJECT_END_TAG,
        HTML_ORDERED_LIST_END_TAG,
        HTML_OPTION_GROUP_END_TAG,
        HTML_OPTION_END_TAG,
        HTML_OUTPUT_END_TAG,
        HTML_PARAGRAPH_END_TAG,
        HTML_PICTURE_END_TAG,
        HTML_PREFORMATED_TEXT_END_TAG,
        HTML_PROGRESS_INDIDCATOR_END_TAG,
        HTML_INLINE_QUOTATION_END_TAG,
        HTML_RUBY_FALLBACK_PARENTHESIS_END_TAG,
        HTML_RUBY_TEXT_END_TAG,
        HTML_RUBY_ANNOTATION_END_TAG,
        HTML_STRIKE_THROUGH_END_TAG,
        HTML_SAMPLE_OUTPUT_END_TAG,
        HTML_SCRIPT_END_TAG,
        HTML_GENERIC_SEARCH_END_TAG,
        HTML_GENERIC_SECTION_END_TAG,
        HTML_SELECT_END_TAG,
        HTML_WEB_COMPONENT_SLOT_END_TAG,
        HTML_SIDE_COMMENT_SMALL_PRINT_END_TAG,
        HTML_CONTENT_SPAN_END_TAG,
        HTML_STRONG_IMPORTANCE_END_TAG,
        HTML_STYLE_INFORMATION_END_TAG,
        HTML_SUBSCRIPT_END_TAG,
        HTML_DISCLOSURE_SUMMARY_END_TAG,
        HTML_SUPERSCRIPT_END_TAG,
        HTML_TABLE_END_TAG,
        HTML_TABLE_BODY_END_TAG,
        HTML_TABLE_DATA_CELL_END_TAG,
        HTML_CONTENT_TEMPLATE_END_TAG,
        HTML_TEXT_AREA_END_TAG,
        HTML_TABLE_FOOTER_END_TAG,
        HTML_TABLE_HEADER_END_TAG,
        HTML_TABLE_HEAD_END_TAG,
        HTML_DATE_TIME_END_TAG,
        HTML_DOCUMENT_TITLE_END_TAG,
        HTML_TABLE_ROW_END_TAG,
        HTML_UNARTICULATED_ANNOTATION_END_TAG,
        HTML_UNORDERED_LIST_END_TAG,
        HTML_VARIABLE_END_TAG,
        HTML_VIDEO_EMBED_END_TAG,
    ]);

    return hypertext_markup_end_tags;
}
