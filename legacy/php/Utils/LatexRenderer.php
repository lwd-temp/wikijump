<?php

namespace Wikidot\Utils;

class LatexRenderer
{
    private $latexPath = "/usr/bin/latex";
    private $dvipsPath = "/usr/bin/dvips";
    private $convertPath = "/usr/bin/convert";
    private $tmpDir = "/tmp/latex-renderer";
    private $outputDir = "/usr/home/barik/public_html/gehennom/lj/cache";

    private $density = 130;

    public function wrap($thunk)
    {
        return<<<EOS
\documentclass[10pt]{article}
% add additional packages here
\usepackage{amsmath}
\usepackage{amsfonts}
\usepackage{amssymb}
\usepackage{pst-plot}
\usepackage{color}
\pagestyle{empty}
\begin{document}
$thunk
\end{document}
EOS;
    }

    function render($thunk, $hash)
    {
        $thunk = $this->wrap($thunk);
        $current_dir = getcwd();
        chdir($this->tmpDir);
        // create temporary LaTeX file
        $fp = fopen($this->tmpDir."/$hash.tex", "w+");
        fputs($fp, $thunk);
        fclose($fp);
        // run LaTeX to create temporary DVI file
        $command = $this->latexPath." --interaction=nonstopmode ".$hash.".tex";
        exec($command);
        if (!file_exists($hash.".dvi")) {
            return false;
        }
        // run dvips to create temporary PS file
        $command = $this->dvipsPath." -E $hash".".dvi -o "."$hash.eps";
        exec($command);
        // run PS file through ImageMagick to
        // create PNG file
        $command = $this->convertPath." -verbose -density ".$this->density." ".$hash.".eps ".$hash.".png 2>&1";

        exec($command, $out);

        // copy the file to the cache directory
        if (!file_exists($hash.".png")) {
            return false;
        }
        copy("$hash.png", $this->outputDir."/$hash.png");
        chdir($current_dir);
        $this->cleanup($hash);
    }

    function cleanup($hash)
    {

        unlink($this->tmpDir."/$hash.tex");
        unlink($this->tmpDir."/$hash.aux");
        unlink($this->tmpDir."/$hash.log");
        unlink($this->tmpDir."/$hash.dvi");
        unlink($this->tmpDir."/$hash.eps");
        unlink($this->tmpDir."/$hash.png");
    }

    public function setTmpDir($tmpdir)
    {
        $this->tmpDir = $tmpdir;
    }

    public function setOutputDir($outdir)
    {
        $this->outputDir = $outdir;
    }

    public function setDensity($val)
    {
        $this->density = $val;
    }
}