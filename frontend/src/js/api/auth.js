export const submitAuthForm = async (formElement, endpoint) => {
  const formData = new FormData(formElement);

  const data = Object.fromEntries(formData.entries());

  try {
    const response = await fetch(`http://localhost:8000/${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.message || `HTTP Error: ${response.status}`);
    }

    return await response.json();

  } catch (error) {
    console.error(`Failed to post to ${endpoint}:`, error);
    throw error;
  }
};

