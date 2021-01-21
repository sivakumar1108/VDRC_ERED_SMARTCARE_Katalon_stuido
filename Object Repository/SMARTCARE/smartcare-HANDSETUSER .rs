<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>smartcare-HANDSETUSER </name>
   <tag></tag>
   <elementGuidId>a791715e-ba9f-4cfb-bad6-066e4b1e3db9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:v1\u003d\&quot;http://seq.huawei.com/interface/dsi/v1.1\&quot;\u003e\n   \u003csoapenv:RequestHeader\u003e\n        \u003creqTime\u003e2020-12-02 11:35:30\u003c/reqTime\u003e\n        \u003c/soapenv:RequestHeader\u003e\n   \u003csoapenv:RequestBody\u003e\n        \u003cHandsetUserList\u003e\n        \u003cUser\u003e\n           \u003cstarttime\u003e2020-12-02 15:33:54\u003c/starttime\u003e\n           \u003cmsisdn\u003e123456782\u003c/msisdn\u003e\n           \u003cimsi\u003e630010442555323\u003c/imsi\u003e\n            \u003cimei\u003e359826104397703\u003c/imei\u003e\n            \u003ccell_id\u003e630012395BCF5\u003c/cell_id\u003e\n            \u003clayer2id\u003e8\u003c/layer2id\u003e\n            \u003csrv_type\u003eLocation Update\u003c/srv_type\u003e\n            \u003crat\u003e1\u003c/rat\u003e\n            \u003cnew_brand\u003eSAMSUNG\u003c/new_brand\u003e\n            \u003cnew_model\u003eSM-A260G(GALAXY A2 CORE)\u003c/new_model\u003e\n            \u003cold_brand\u003eTECNO\u003c/old_brand\u003e\n            \u003cold_model\u003eT312\u003c/old_model\u003e\n            \u003cevent_id\u003e8004\u003c/event_id\u003e\n    \u003cevent_type\u003eCEP_ypeijia_20200429120844195\u003c/event_type\u003e\n        \u003c/User\u003e\n        \u003c/HandsetUserList\u003e\n   \u003c/soapenv:RequestBody\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
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
   <restUrl>http://10.200.61.126:8098/SmartCarePlugin/WSDLLoader?group=HANDSETUSER</restUrl>
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
