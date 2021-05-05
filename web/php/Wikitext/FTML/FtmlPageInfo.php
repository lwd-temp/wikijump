<?php
declare(strict_types = 1);

namespace Wikidot\Wikitext\FTML;

use \FFI;

/**
 * Class FtmlPageInfo, representing an input 'struct ftml_page_info' object.
 * @package Wikidot\Wikitext\FTML
 */
class FtmlPageInfo
{
    private FFI\CData $c_data;

    public function __construct(
        string $page,
        ?string $category,
        string $site,
        string $title,
        ?string $alt_title,
        array $tags,
        string $locale
    ) {
        $tag_array = FtmlRaw::listToPointer(
            FtmlRaw::$C_STRING,
            $tags,
            fn(string $tag) => FtmlRaw::string($tag),
        );

        $this->c_data = FtmlRaw::make(FtmlRaw::$FTML_PAGE_INFO);
        $this->c_data->page = FtmlRaw::string($page);
        $this->c_data->category = FtmlRaw::string($category);
        $this->c_data->site = FtmlRaw::string($site);
        $this->c_data->title = FtmlRaw::string($title);
        $this->c_data->alt_title = FtmlRaw::string($alt_title);
        $this->c_data->tags_list = $tag_array->pointer;
        $this->c_data->tags_len = $tag_array->length;
        $this->c_data->locale = FtmlRaw::string($locale);
    }

    public function pointer(): FFI\CData {
        return FFI::addr($this->c_data);
    }

    function __destruct() {
        FFI::free($this->c_data->page);
        FFI::free($this->c_data->category);
        FFI::free($this->c_data->site);
        FFI::free($this->c_data->title);
        FFI::free($this->c_data->alt_title);
        FtmlRaw::freePointer(
            $this->c_data->tags_list,
            $this->c_data->tags_len,
            fn(FFI\CData $c_data) => FFI::free($c_data),
        );
        FFI::free($this->c_data->locale);
        FFI::free($this->c_data);
    }
}