import pandas as pd
import matplotlib.pyplot as plt

# Load the CSV data
data = pd.read_csv("multi_arm_bandit_files/exploration_exploitation_log.csv")

# Ensure 'Exploitation' and 'Exploration' are numeric
data['Exploitation'] = pd.to_numeric(data['Exploitation'], errors='coerce')
data['Exploration'] = pd.to_numeric(data['Exploration'], errors='coerce')

# Check if there are any missing values
print(data.isnull().sum())

# Group data by 'Transaction' and 'Connector' for plotting
grouped = data.groupby(["Transaction", "Connector"])[['Exploitation', 'Exploration']].mean().reset_index()

# List of connectors to plot
connectors = ['cybersource  ', 'adyen        ', 'bankofamerica']

# Create subplots for each connector
fig, axes = plt.subplots(len(connectors), 1, figsize=(14, 12), sharex=True)

# Plot curves for each connector
for i, connector in enumerate(connectors):
    connector_data = grouped[grouped['Connector'] == connector]

    # If there is data for this connector
    if not connector_data.empty:
        ax = axes[i]
        ax.plot(connector_data['Transaction'], connector_data['Exploitation'],
                label=f"{connector.strip()} - Exploitation", linestyle='-', marker='o', alpha=0.7, linewidth=2)
        ax.plot(connector_data['Transaction'], connector_data['Exploration'],
                label=f"{connector.strip()} - Exploration", linestyle='--', marker='x', alpha=0.7, linewidth=2)

        ax.set_title(f"{connector.strip()} - Exploration vs Exploitation", fontsize=14)
        ax.set_ylabel("Score", fontsize=12)
        ax.grid(True, which='both', linestyle='--', linewidth=0.5)
        ax.legend(loc='center right', fontsize=10)

# Set the common x-label
plt.xlabel("Number of Transactions", fontsize=14)

# Adjust layout for better spacing
plt.tight_layout()

# Save the diagram to a file
output_file = "multi_arm_bandit_files/exploration_vs_exploitation_connectors.png"
plt.savefig(output_file, format="png", dpi=300)
print(f"Plot saved to {output_file}")
