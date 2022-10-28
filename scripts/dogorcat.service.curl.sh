#!/bin/bash
set -e
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
APP_DIR="$(dirname "$SCRIPT_DIR")"

echo "CURL Testing for DOGORCAT-SERVICE"
echo

echo "1) Upload and set Dataset for Classification"
curl -F data="@${APP_DIR}/models/dogorcat.zip;type=application/zip" -F latest='true' http://localhost:8000/dataset
echo; echo

echo "2.1) Classify cat1.jpg (Image of a Cat)"
curl -F image="@${APP_DIR}/images/cat1.jpg" http://localhost:8000/dogorcat
echo

echo "2.2) Classify cat2.jpg (Image of a Cat)"
curl -F image="@${APP_DIR}/images/cat2.jpg" http://localhost:8000/dogorcat
echo

echo "2.3) Classify cat3.jpg (Image of a Cat)"
curl -F image="@${APP_DIR}/images/cat3.jpg" http://localhost:8000/dogorcat
echo; echo

echo "3.1) Upload Dataset for Classification without setting it"
DATASET_ID="$(curl -F data="@${APP_DIR}/models/dogorcat.zip;type=application/zip" http://localhost:8000/dataset | jq -r '.id')"

echo "3.2) Set previously uploaded Dataset"
curl -X PUT "http://localhost:8000/dataset/${DATASET_ID}"
echo "Success"; echo

echo "4.1) Classify dog1.jpg (Image of a Dog)"
curl -F image="@${APP_DIR}/images/dog1.jpg" http://localhost:8000/dogorcat
echo

echo "4.2) Classify dog2.jpg (Image of a Dog)"
curl -F image="@${APP_DIR}/images/dog2.jpg" http://localhost:8000/dogorcat
echo

echo "4.3) Classify dog3.jpg (Image of a Dog)"
curl -F image="@${APP_DIR}/images/dog3.jpg" http://localhost:8000/dogorcat
echo; echo

echo "5) Delete all Datasets"
curl -X DELETE http://localhost:8000/datasets
echo