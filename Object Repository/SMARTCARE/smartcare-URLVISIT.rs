<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>smartcare-URLVISIT</name>
   <tag></tag>
   <elementGuidId>644173be-89d3-4c3c-be64-d3e82cb23806</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:v1\u003d\&quot;http://seq.huawei.com/interface/dsi/v1.1\&quot;\u003e\n\u003csoapenv:RequestHeader\u003e\n            \u003creqTime\u003e2020-02-14 00:04:00\u003c/reqTime\u003e\n            \u003c/soapenv:RequestHeader\u003e\n\u003csoapenv:RequestBody\u003e\n\u003cURLVisitUserList\u003e  \n\u003cUser\u003e\n\u003cstarttime\u003e2020-11-06 16:21:44\u003c/starttime\u003e\n\u003cmsisdn\u003e123456789\u003c/msisdn\u003e\n                \u003cimsi\u003e630010244441713\u003c/imsi\u003e\n                \u003cimei\u003e359554107627161\u003c/imei\u003e\n                \u003ccell_id\u003e63001032D107\u003c/cell_id\u003e\n                \u003clayer2id\u003e\u003c/layer2id\u003e\n                \u003ctraffic\u003e11229\u003c/traffic\u003e\n                \u003capp_id\u003e342\u003c/app_id\u003e\n                \u003chost\u003e\u003c/host\u003e\n\u003cprot_type\u003e1929\u003c/prot_type\u003e\n                \u003csrv_type\u003e\u003c/srv_type\u003e\n                \u003crat\u003e6\u003c/rat\u003e\n                \u003cevent_id\u003e8004\u003c/event_id\u003e\n                                                       \u003c/User\u003e\n                                       \n        \u003c/URLVisitUserList\u003e\n\u003c/soapenv:RequestBody\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.200.178.76:8091/SmartCarePlugin/WSDLLoader?group=URLVISIT</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
