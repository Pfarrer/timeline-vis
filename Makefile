CURL = curl -v
# CURL = curl --silent

ES_HOST = localhost:9200
ES_ACTIVITYSEGMENT_INDEX_NAME = activity_segment

CACHE_DIR = run-cache
ORIGINAL_SEMANTICS_DIR = ${CACHE_DIR}/semantics-original
SEPERATED_SEMANTICS_DIR = ${CACHE_DIR}/semantics-seperated
ORIGINAL_LOCATIONS_FILE = ${CACHE_DIR}/locations-original.json
SEPERATED_LOCATIONS_FILE = ${CACHE_DIR}/locations-seperated.json
SEMANTICS_PATH_IN_TAKEOUT_ZIP = Takeout/Standortverlauf/Semantic Location History
LOCATIONS_PATH_IN_TAKEOUT_ZIP = Takeout/Standortverlauf/Standortverlauf.json

.PHONY: clean clean-es unzip-semantics seperate-semantics create-es-index

${CACHE_DIR}:
	mkdir -p ${CACHE_DIR}

${ORIGINAL_SEMANTICS_DIR}: ${CACHE_DIR}
	mkdir -p ${ORIGINAL_SEMANTICS_DIR}

${SEPERATED_SEMANTICS_DIR}: ${CACHE_DIR}
	mkdir -p ${SEPERATED_SEMANTICS_DIR}

unzip-semantics: ${CACHE_DIR} ${ORIGINAL_SEMANTICS_DIR}
	unzip takeout-*.zip '${SEMANTICS_PATH_IN_TAKEOUT_ZIP}/*' -d ${CACHE_DIR}
	find '${CACHE_DIR}/${SEMANTICS_PATH_IN_TAKEOUT_ZIP}' -name "*.json" -print0 | xargs -0 -I {} mv {} ${ORIGINAL_SEMANTICS_DIR}
	rm -r '${CACHE_DIR}/Takeout'

seperate-semantics: unzip-semantics ${SEPERATED_SEMANTICS_DIR}
	sh ./seperate-semantics.sh ${ORIGINAL_SEMANTICS_DIR} ${SEPERATED_SEMANTICS_DIR}

${ORIGINAL_LOCATIONS_FILE}: ${CACHE_DIR}
	unzip takeout-*.zip '${LOCATIONS_PATH_IN_TAKEOUT_ZIP}' -d ${CACHE_DIR}
	mv ${CACHE_DIR}/${LOCATIONS_PATH_IN_TAKEOUT_ZIP} ${ORIGINAL_LOCATIONS_FILE}
	rm -r '${CACHE_DIR}/Takeout'

${SEPERATED_LOCATIONS_FILE}: ${ORIGINAL_LOCATIONS_FILE}
	jq -c '.locations[]' ${ORIGINAL_LOCATIONS_FILE} > ${SEPERATED_LOCATIONS_FILE}

index: seperate-semantics ${SEPERATED_LOCATIONS_FILE}

clean:
	rm -r $(CACHE_DIR)

clean-es:
	$(CURL) -XDELETE $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME) > /dev/null

create-es-index : clean-es es-mapping-activitysegment.json
	$(CURL) -XPUT $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME) > /dev/null
	$(CURL) -XPUT $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME)/_mapping -H 'Content-Type: application/json' --data-binary "@es-mapping-activitysegment.json" | jq

debug: | clean-es create-es-index
	cargo run -- run-cache/locations-seperated.json run-cache/semantics-seperated/2020_JANUARY.json run-cache/semantics-seperated/2020_FEBRUARY.json run-cache/semantics-seperated/2020_MARCH.json > /tmp/a
	curl -XPOST localhost:9200/_bulk -H 'Content-Type: application/json' --data-binary @/tmp/a | jq

# CURL = curl
# CURL = curl --silent

# ES_HOST = localhost:9200
# ES_INDEX_NAME = testindex

# TEMP_FOLDER = run-cache
# JQ_TEMP_FILE = $(TEMP_FOLDER)/jq-result.json
# ENRICH_TEMP_FILE = $(TEMP_FOLDER)/enrichment-result.json
# SPLIT_FOLDER = $(TEMP_FOLDER)/chunks

# run : cargo-build es-create-index es-index-chunk

# clean : es-clean
# 	rm $(SPLIT_FOLDER)/*
# 	rmdir $(SPLIT_FOLDER)
# 	rm $(TEMP_FOLDER)/*
# 	rmdir $(TEMP_FOLDER)

# make-temp-folder :
# 	mkdir -p $(TEMP_FOLDER)
# 	mkdir -p $(SPLIT_FOLDER)

# cargo-build : src/*
# 	cargo build --release

# es-clean :
# 	$(CURL) -XDELETE $(ES_HOST)/$(ES_INDEX_NAME) > /dev/null

# es-create-index : es-clean es-index.json
# 	$(CURL) -XPUT $(ES_HOST)/$(ES_INDEX_NAME) > /dev/null
# 	$(CURL) -XPUT $(ES_HOST)/$(ES_INDEX_NAME)/_mapping -H 'Content-Type: application/json' --data-binary "@es-index.json" | jq

# jq : make-temp-folder $(TAKEOUT_JSON)
# 	cat $(TAKEOUT_JSON) | jq -c '.locations[]' > $(JQ_TEMP_FILE)

# enrich : make-temp-folder jq
# 	cat $(JQ_TEMP_FILE) | ./target/release/timeline-vis > $(ENRICH_TEMP_FILE)

# es-index-chunk : make-temp-folder enrich
# 	split -l 2000 -a 5 $(ENRICH_TEMP_FILE) $(SPLIT_FOLDER)/
# # 	echo "\n" >> $^
# #     $(foreach file, $(wildcard $(SPLIT_FOLDER)/*), $(CURL) -XPOST $(ES_HOST)/_bulk -H 'Content-Type: application/json' --data-binary "@$(file)";)
# 	./index-chunks.sh
