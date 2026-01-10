# Axiom Jupyter Notebook Examples

Interactive Jupyter notebooks demonstrating semantic analysis with Axiom.

## Notebooks

### semantic_analysis.ipynb

Complete semantic analysis workflow:
- Connect to Axiom
- Create and manage tokens
- Visualize embeddings with t-SNE
- Semantic search
- Document clustering with K-means
- Similarity matrix analysis

## Setup

1. Install dependencies:
```bash
pip install axiom-python jupyter matplotlib scikit-learn pandas numpy seaborn
```

2. Start Jupyter:
```bash
jupyter notebook
```

3. Open `semantic_analysis.ipynb`

## Requirements

- Python 3.10+
- Axiom API running at http://localhost:8000
- Jupyter Notebook or JupyterLab

## Features Demonstrated

- **Data Visualization**: t-SNE dimensionality reduction
- **Clustering**: K-means clustering of semantic embeddings
- **Similarity Analysis**: Cosine similarity heatmaps
- **Semantic Search**: Finding similar documents
- **Interactive Analysis**: Exploratory data analysis workflows

## Running in Google Colab

Upload the notebook to Google Colab and install dependencies:

```python
!pip install axiom-python matplotlib scikit-learn pandas seaborn
```

Note: Update the Axiom API URL to a publicly accessible endpoint.
