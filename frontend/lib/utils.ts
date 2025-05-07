import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import {Server} from "stellar-sdk"




export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function truncateAddress(address: string, start = 6, end = 6): string {
  if (!address) return ""
  if (address.length <= start + end) return address
  return `${address.slice(0, start)}...${address.slice(-end)}`
}

export function formatTokenAmount(amount: string): string {
  return Number.parseFloat(amount).toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  })
}

export function formatDate(date: Date): string {
  // If less than 24 hours ago, show relative time
  const now = new Date()
  const diff = now.getTime() - date.getTime()

  if (diff < 1000 * 60) {
    return "Just now"
  } else if (diff < 1000 * 60 * 60) {
    const minutes = Math.floor(diff / (1000 * 60))
    return `${minutes} minute${minutes !== 1 ? "s" : ""} ago`
  } else if (diff < 1000 * 60 * 60 * 24) {
    const hours = Math.floor(diff / (1000 * 60 * 60))
    return `${hours} hour${hours !== 1 ? "s" : ""} ago`
  } else {
    // Format as MM/DD/YYYY
    return date.toLocaleDateString()
  }
}


const server = new Server("https://horizon-testnet.stellar.org");

export async function fetchTotalSupply(assetCode: string, issuer: string): Promise<string> {
  try {
    if (!assetCode || !issuer) {
      throw new Error("Invalid assetCode or issuer. Both parameters are required.");
    }

    console.log("Fetching total supply for:", { assetCode, issuer });

    // First, check if we're using the correct server endpoint
    const server = new Server("https://horizon-testnet.stellar.org"); // Changed to testnet based on your account

    // Try to get the asset info
    const response = await server.assets()
      .forCode(assetCode)
      .forIssuer(issuer)
      .call();

    console.log("API Response:", response);

    if (response.records && response.records.length > 0) {
      const totalSupply = response.records[0]?.amount || "0";
      return totalSupply;
    } else {
      console.log("Asset not found. It may not exist yet or has no trustlines.");
      return "0";
    }
  } catch (error:any) {
    console.error("Failed to fetch total supply:", error);
    
    // More detailed error handling
    if (error.response && error.response.status === 404) {
      console.log("Asset not found. It may not have been created yet or has no trustlines.");
    }
    
    return "0";
  }
}