.PHONY: dict

FULL_DICT ?= /usr/share/dict/words

dict:
	cd makedict && cargo run -- $(FULL_DICT) ../src/fixtures/words

dict_test:
	cd makedict && cargo test
