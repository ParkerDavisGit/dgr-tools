import window
import logging
import sys


logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG)
handler = logging.StreamHandler(sys.stderr)
handler.setFormatter(
    logging.Formatter("%(name)s".center(10)+" - %(levelname)s - %(message)s")
    #logging.Formatter("%(name)s - %(levelname)s - %(message)s")
)
logger.addHandler(handler)

def get_dgrlin_logger():
    logger = logging.getLogger("dgrlin")
    logger.setLevel(logging.DEBUG)
    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(
        logging.Formatter("%(name)s".center(12)+" - %(levelname)s - %(message)s")
        #logging.Formatter("%(name)s - %(levelname)s - %(message)s")
    )
    logger.addHandler(handler)

# dgrlin.compile("decompile")

def main():
    get_dgrlin_logger()
    logger.info("starting program")
    window.Window()
    logger.info("closing program")


if __name__ == "__main__":
    main()