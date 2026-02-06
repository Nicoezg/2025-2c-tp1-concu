#!/bin/bash

mkdir -p data

echo "Downloading NYC taxi dataset from Kaggle..."

kaggle datasets download -d elemento/nyc-yellow-taxi-trip-data -p data --unzip

echo "Download complete."
