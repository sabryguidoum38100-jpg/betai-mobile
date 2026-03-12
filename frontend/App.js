import React, { useState } from 'react';
import { StyleSheet, Text, View, Button, TextInput } from 'react-native';

export default function App() {
  const [matchId, setMatchId] = useState('');
  const [prediction, setPrediction] = useState(null);

  const fetchPrediction = async () => {
    try {
      const response = await fetch(`https://ton-api-url.com/predict/${matchId}`);
      const data = await response.json();
      setPrediction(data);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <View style={styles.container}>
      <Text style={styles.title}>BetAI Mobile</Text>
      <TextInput
        style={styles.input}
        placeholder="Entrez l'ID du match"
        onChangeText={setMatchId}
        value={matchId}
      />
      <Button title="Obtenir la prédiction" onPress={fetchPrediction} />
      {prediction && (
        <View style={styles.result}>
          <Text>Conseil : {prediction.recommended_bet}</Text>
          <Text>Confiance : {prediction.confidence}%</Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#fff', alignItems: 'center', justifyContent: 'center' },
  title: { fontSize: 24, marginBottom: 20 },
  input: { borderWidth: 1, padding: 10, width: 200, marginBottom: 10 },
  result: { marginTop: 20 }
});
