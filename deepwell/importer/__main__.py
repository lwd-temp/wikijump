#!/usr/bin/env python3

import argparse
import logging
import os
import sys

from .importer import Importer

LOG_FORMAT = "[%(levelname)s] [%(asctime)s] %(message)s"
LOG_DATE_FORMAT = "%Y/%m/%d %H:%M:%S"

if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description="WikiComma importer")
    argparser.add_argument(
        "-q",
        "--quiet",
        "--no-stdout",
        dest="stdout",
        action="store_false",
        help="Don't output to standard out",
    )
    argparser.add_argument(
        "-D",
        "--debug",
        dest="debug",
        action="store_true",
        help="Set logging level to debug",
    )
    argparser.add_argument(
        "-d",
        "--directory",
        "--wikicomma-directory",
        dest="wikicomma_directory",
        required=True,
        help="The directory where WikiComma data resides",
    )
    argparser.add_argument(
        "-o",
        "--sqlite",
        "--output-sqlite",
        dest="sql_path",
        required=True,
        help="The location to output the SQLite database to",
    )
    argparser.add_argument(
        "-b",
        "--bucket",
        "--s3-bucket",
        dest="s3_bucket",
        required=True,
        help="The S3 bucket to store uploaded files in",
    )
    argparser.add_argument(
        "-P",
        "--profile",
        "--aws-profile",
        dest="aws_profile",
        required=True,
        help="The AWS profile containing the secrets",
    )
    args = argparser.parse_args()

    log_fmtr = logging.Formatter(LOG_FORMAT, datefmt=LOG_DATE_FORMAT)
    log_stdout = logging.StreamHandler(sys.stdout)
    log_stdout.setFormatter(log_fmtr)
    log_level = logging.DEBUG if args.debug else logging.INFO

    logger = logging.getLogger("importer")
    logger.setLevel(level=log_level)
    logger.addHandler(log_stdout)

    importer = Importer(
        logger=logger,
        wikicomma_directory=args.wikicomma_directory,
        sqlite_path=args.sqlite_path,
        aws_profile=args.aws_profile,
    )
    importer.run()
