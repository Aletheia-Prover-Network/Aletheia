import axios from 'axios';
import fs from 'fs';
import dotenv from 'dotenv';
dotenv.config();

const API_URL = 'https://relayer-api.horizenlabs.io/api/v1';

const proof = JSON.parse(fs.readFileSync("../aletheia/host/proof_output.json"));

console.log("this is the proof", proof.image_id);



async function main() {
    // Required code
    if (!fs.existsSync("r0-vkey.json")) {
        // Registering the verification key
        try {
            const regParams = {
                "proofType": "risc0",
                "proofOptions": {
                    "version": "V2_1"
                },
                "vk": proof.image_id
            }
            
            const regResponse = await axios.post(`${API_URL}/register-vk/${process.env.API_KEY}`, regParams);
            fs.writeFileSync(
                "r0-vkey.json",
                JSON.stringify(regResponse.data)
            );
        } catch (error) {
            fs.writeFileSync(
                "r0-vkey.json",
                JSON.stringify(error.response.data)
            );
        }
    }

    const vk = JSON.parse(fs.readFileSync("r0-vkey.json"));

    const params = {
        "proofType": "risc0",
        "vkRegistered": true,
        "proofOptions": {
            "version": "V2_1"
        },
        "proofData": {
            "proof": proof.proof,
            "publicSignals": proof.pub_inputs,
            "vk": vk.vkHash || vk.meta.vkHash
        }
    }

    console.log("log api key here", process.env.API_KEY)
    const requestResponse = await axios.post(`${API_URL}/submit-proof/${process.env.API_KEY}`, params)
    console.log("This is the request response", requestResponse.data)

    if (requestResponse.data.optimisticVerify != "success") {
        console.error("Proof verification, check proof artifacts");
        return;
    }

    while (true) {
        const jobStatusResponse = await axios.get(`${API_URL}/job-status/${process.env.API_KEY}/${requestResponse.data.jobId}`);
        if (jobStatusResponse.data.status === "Finalized") {
            console.log("Job finalized successfully");
            console.log(jobStatusResponse.data);
            break;
        } else {
            console.log("Job status: ", jobStatusResponse.data.status);
            console.log("Waiting for job to finalize...");
            await new Promise(resolve => setTimeout(resolve, 5000)); // Wait for 5 seconds before checking again
        }
    }
}

main();