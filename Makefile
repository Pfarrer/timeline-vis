CURL = curl -v
# CURL = curl --silent

ES_HOST = localhost:9200
ES_ACTIVITYSEGMENT_INDEX_NAME = activitysegment

CACHE_DIR = run-cache
ORIGINAL_ACTIVITIES_DIR = ${CACHE_DIR}/original
ENRICHED_ACTIVITIES_DIR = ${CACHE_DIR}/enriched
SPLITTED_ACTIVITIES_DIR = ${CACHE_DIR}/splitted
PATH_IN_TAKEOUT_ZIP = Takeout/Standortverlauf/Semantic Location History

.PHONY: clean clean-es unzip-files seperate-activities create-es-index

index: seperate-activities

${CACHE_DIR}:
	mkdir -p ${CACHE_DIR}

${ORIGINAL_ACTIVITIES_DIR}: ${CACHE_DIR}
	mkdir -p ${ORIGINAL_ACTIVITIES_DIR}

${ENRICHED_ACTIVITIES_DIR}: ${CACHE_DIR}
	mkdir -p ${ENRICHED_ACTIVITIES_DIR}

${SPLITTED_ACTIVITIES_DIR}: ${CACHE_DIR}
	mkdir -p ${SPLITTED_ACTIVITIES_DIR}

unzip-files: ${CACHE_DIR} ${ORIGINAL_ACTIVITIES_DIR} takeout-*.zip
	unzip takeout-*.zip '${PATH_IN_TAKEOUT_ZIP}/*' -d ${CACHE_DIR}
	find '${CACHE_DIR}/${PATH_IN_TAKEOUT_ZIP}' -name "*.json" -print0 | xargs -0 -I {} mv {} ${ORIGINAL_ACTIVITIES_DIR}
	rm -r '${CACHE_DIR}/Takeout'

seperate-activities: unzip-files ${ENRICHED_ACTIVITIES_DIR}
	sh ./enrich-activities.sh ${ORIGINAL_ACTIVITIES_DIR} ${ENRICHED_ACTIVITIES_DIR}

clean:
	rm -r $(CACHE_DIR)

clean-es:
	$(CURL) -XDELETE $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME) > /dev/null

create-es-index : clean-es es-mapping-activitysegment.json
	$(CURL) -XPUT $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME) > /dev/null
	$(CURL) -XPUT $(ES_HOST)/$(ES_ACTIVITYSEGMENT_INDEX_NAME)/_mapping -H 'Content-Type: application/json' --data-binary "@es-mapping-activitysegment.json" | jq

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
