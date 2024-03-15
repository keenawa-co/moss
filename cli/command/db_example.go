package command

import (
	"context"
	"encoding/json"
	"log"
	"time"

	clientv3 "go.etcd.io/etcd/client/v3"
	"go.etcd.io/etcd/server/v3/embed"
)

func dbExample() {
	cfg := embed.NewConfig()
	cfg.Dir = ".ray/cache"

	e, err := embed.StartEtcd(cfg)
	if err != nil {
		log.Fatal(err)
	}
	defer e.Close()

	select {
	case <-e.Server.ReadyNotify():
		log.Println("Server is ready!")
	case <-time.After(60 * time.Second):
		e.Server.Stop()
		log.Fatal("Server took too long to start!")
	}

	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   []string{"localhost:2379"},
		DialTimeout: 5 * time.Second,
	})
	if err != nil {
		log.Fatal(err)
	}
	defer cli.Close()

	jsData, _ := json.Marshal(map[string]string{
		"key1": "value1",
	})

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	_, err = cli.Put(ctx, "your/key", string(jsData))
	cancel()
	if err != nil {
		log.Fatal(err)
	}
}
